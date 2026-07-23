use std::collections::HashMap;

/// Substitute `{param}` placeholders in a URL template with values.
///
/// # Example
/// ```
/// # use amazon_ad_api::apis::helpers::fill_query_params;
/// assert_eq!(
///     fill_query_params("/v2/sp/campaigns/{campaignId}", &[("campaignId", "123456")]),
///     "/v2/sp/campaigns/123456"
/// );
/// ```
pub fn fill_query_params(template: &str, params: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in params {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

/// Convert a flat `HashMap` with dot-separated keys into a nested `serde_json::Value`.
///
/// # Example
/// ```ignore
/// let mut flat = HashMap::new();
/// flat.insert("a.b.c".to_string(), "value".to_string());
/// let nested = nest_dict(flat);
/// // nested = {"a": {"b": {"c": "value"}}}
/// ```
pub fn nest_dict(flat: HashMap<String, String>) -> serde_json::Value {
    let mut root = serde_json::Map::new();

    for (key, value) in flat {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &mut root;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.insert(part.to_string(), serde_json::Value::String(value.clone()));
            } else {
                current = current
                    .entry(part.to_string())
                    .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .expect("nest_dict: intermediate key is not an object");
            }
        }
    }

    serde_json::Value::Object(root)
}

// ── Request execution with rate limiting ──────────────────────

use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use std::collections::HashMap as StdHashMap;

/// Execute a `reqwest::Request` with rate limiting and proper header extraction.
///
/// This is the central execution path for all Advertising API calls.
/// It handles:
/// 1. Token-bucket rate limiting (waits before executing, keyed by URL path)
/// 2. Dynamic rate limit updates from `x-ad-api-rate-limit-*` headers
/// 3. Proper extraction of `next_token` from headers and JSON body
/// 4. Full response header capture
pub async fn execute_request<T: serde::de::DeserializeOwned>(
    configuration: &Configuration,
    request: reqwest::Request,
) -> Result<ApiResponse<T>, Error<serde_json::Value>> {
    // Derive endpoint key from the URL path
    let endpoint_key = derive_endpoint_key(request.url().path());
    let (default_rate, default_burst) = rate_limit_for_path(request.url().path());

    // 1. Wait on rate limiter
    let rate_limit_guard = if let Some(ref rl) = configuration.rate_limiter {
        Some(rl.wait(&endpoint_key, default_rate, default_burst).await)
    } else {
        None
    };

    // 2. Execute the HTTP request
    let resp = configuration.client.execute(request).await?;
    let status = resp.status();

    // 3. Extract relevant headers before consuming the body
    let mut headers = StdHashMap::new();
    for (name, value) in resp.headers().iter() {
        let name_str = name.as_str();
        if name_str.starts_with("x-")
            || name_str.starts_with("X-")
            || name_str.eq_ignore_ascii_case("nexttoken")
            || name_str.eq_ignore_ascii_case("location")
            || name_str.eq_ignore_ascii_case("content-type")
        {
            if let Ok(v) = value.to_str() {
                headers.insert(name_str.to_lowercase(), v.to_string());
            }
        }
    }

    // Record response time for rate limiter
    if let Some(guard) = rate_limit_guard {
        guard.mark_response();
    }

    // 4. Update rate limiter from response headers
    if let Some(ref rl) = configuration.rate_limiter {
        let parsed_rate = headers
            .get("x-ad-api-rate-limit-limit")
            .and_then(|v| v.parse::<f64>().ok());
        let parsed_burst = parsed_rate.map(|r| r as u32);
        if let (Some(rate), Some(burst)) = (parsed_rate, parsed_burst) {
            rl.update_limits(&endpoint_key, rate, burst).await;
        }
    }

    // 5. Read response body
    let body_str = resp.text().await?;

    if status.is_success() {
        let payload: T = serde_json::from_str(&body_str)?;
        Ok(ApiResponse::from_parts(payload, headers, &body_str))
    } else {
        let entity = serde_json::from_str::<serde_json::Value>(&body_str).ok();
        Err(Error::ResponseError(crate::apis::ResponseContent {
            status,
            content: body_str,
            entity,
        }))
    }
}

/// Derive a short endpoint key from a URL path for rate limiting.
fn derive_endpoint_key(path: &str) -> String {
    // e.g. /sp/campaigns/list → sp-campaigns
    //      /sb/v4/campaigns   → sb-campaigns
    //      /sd/adGroups        → sd-adGroups
    let segments: Vec<&str> = path.trim_matches('/').split('/').collect();
    if segments.len() >= 2 {
        let prefix = segments[0]; // sp, sb, sd, dsp, v2, v3
        let endpoint = if prefix == "v2" || prefix == "v3" {
            // /v2/sp/... or /v3/reports
            segments
                .get(2)
                .copied()
                .unwrap_or(segments.get(1).copied().unwrap_or("unknown"))
        } else {
            segments[1] // /sp/campaigns → campaigns
        };
        format!("{}-{}", prefix, endpoint)
    } else {
        path.trim_matches('/').to_string()
    }
}

/// Return (rate, burst) for a given URL path.
fn rate_limit_for_path(path: &str) -> (f64, u32) {
    if path.contains("/report") || path.contains("/snapshot") {
        (1.0, 1)
    } else if path.contains("/bid/") || path.contains("recommendations") {
        (2.0, 2)
    } else {
        (5.0, 5)
    }
}

/// Build a URL query string from a serializable struct.
///
/// Converts any `serde::Serialize` struct to a properly percent-encoded query
/// string by round-tripping through `serde_json::Value` and then encoding each
/// non-null scalar field. This avoids generating `?field=null` noise.
///
/// Requires the `client` feature (provides the `url` crate).
///
/// # Example
/// ```ignore
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Params { state: Option<String>, count: Option<u32> }
/// let p = Params { state: Some("enabled".into()), count: Some(10) };
/// let qs = amazon_ad_api::apis::helpers::build_query_string(&p).unwrap();
/// assert!(qs.contains("state=enabled"));
/// ```
#[cfg(feature = "client")]
pub fn build_query_string<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let json = serde_json::to_value(value)?;
    let obj = match &json {
        serde_json::Value::Object(m) => m,
        _ => return Ok(String::new()),
    };

    let mut pairs: Vec<String> = Vec::new();
    for (key, val) in obj {
        let encoded_key =
            ::url::form_urlencoded::byte_serialize(key.as_bytes()).collect::<String>();
        match val {
            serde_json::Value::Null => {} // skip null fields
            serde_json::Value::String(s) => {
                let encoded_val =
                    ::url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>();
                pairs.push(format!("{}={}", encoded_key, encoded_val));
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    let s = match item {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    let encoded_val =
                        ::url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>();
                    pairs.push(format!("{}={}", encoded_key, encoded_val));
                }
            }
            other => {
                let s = other.to_string();
                let encoded_val =
                    ::url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>();
                pairs.push(format!("{}={}", encoded_key, encoded_val));
            }
        }
    }

    Ok(pairs.join("&"))
}
