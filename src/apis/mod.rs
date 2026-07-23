use std::fmt;

use serde::Deserialize;

pub mod body;
pub mod configuration;
pub mod helpers;
pub mod macros;
pub mod pagination;
pub mod params;

#[cfg(feature = "cross")]
pub mod cross;
#[cfg(feature = "dsp")]
pub mod dsp;
#[cfg(feature = "sb")]
pub mod sb;
#[cfg(feature = "sd")]
pub mod sd;
#[cfg(feature = "sp")]
pub mod sp;

/// Response content on error.
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

/// Structured Advertising API error detail.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<String>,
}

/// Wrapper for Advertising API error response.
#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    errors: Vec<ApiErrorDetail>,
}

impl<T> fmt::Display for ResponseContent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status code {}", self.status)?;
        if let Ok(api_err) = serde_json::from_str::<ApiErrorResponse>(&self.content) {
            for err in &api_err.errors {
                write!(f, "\n  [{}] {}", err.code, err.message)?;
                if let Some(ref details) = err.details {
                    if !details.is_empty() {
                        write!(f, " — {}", details)?;
                    }
                }
            }
        } else if !self.content.is_empty() {
            write!(f, ", body:\n{}", self.content)?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error<T> {
    #[error("error in reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("error in serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("error in IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("middleware error: {0}")]
    Middleware(String),
    #[error("custom error: {0}")]
    Custom(String),
    #[error("error in response: {0}")]
    ResponseError(ResponseContent<T>),
}

impl<T> From<anyhow::Error> for Error<T> {
    fn from(e: anyhow::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        match e {
            reqwest_middleware::Error::Reqwest(err) => Error::Reqwest(err),
            reqwest_middleware::Error::Middleware(err) => Error::Middleware(err.to_string()),
        }
    }
}

/// URL-encode a string for query parameters.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// Parse a nested object into `deepObject` style query parameters.
pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    Vec::new()
}

#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_deep_object_non_object_safely_returns_empty() {
        let val = serde_json::json!("string_value");
        let res = parse_deep_object("filter", &val);
        assert!(res.is_empty());
    }

    #[test]
    fn test_parse_deep_object_valid_object() {
        let val = serde_json::json!({ "state": "enabled" });
        let res = parse_deep_object("filter", &val);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], ("filter[state]".to_string(), "enabled".to_string()));
    }
}
