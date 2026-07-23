use reqwest::{header::HeaderMap, Method};

/// Helper struct for building and executing API endpoint requests.
///
/// Replaces the `@sp_endpoint` decorator pattern from the Python library.
/// Provides a strongly-typed, builder-oriented approach to constructing
/// HTTP requests for Amazon Advertising API endpoints.
///
/// # Example
/// ```ignore
/// let response = EndpointRequest::get("/v2/sp/campaigns")
///     .with_header("Accept", "application/vnd.spCampaign.v3+json")
///     .execute::<Campaign>(&config)
///     .await?;
/// ```
pub struct EndpointRequest {
    pub path: String,
    pub method: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
    pub body: Option<String>,
    pub query: Vec<(String, String)>,
}

impl EndpointRequest {
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            method,
            headers: HeaderMap::new(),
            body: None,
            query: Vec::new(),
        }
    }

    pub fn get(path: impl Into<String>) -> Self {
        Self::new(Method::GET, path)
    }

    pub fn post(path: impl Into<String>) -> Self {
        Self::new(Method::POST, path)
    }

    pub fn put(path: impl Into<String>) -> Self {
        Self::new(Method::PUT, path)
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self::new(Method::DELETE, path)
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(
                key.parse::<reqwest::header::HeaderName>().unwrap(),
                value.parse().unwrap(),
            );
        self
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn with_query(mut self, key: &str, value: &str) -> Self {
        self.query.push((key.to_string(), value.to_string()));
        self
    }

    /// Build the full URL from base path and endpoint path.
    pub fn build_url(&self, base_url: &str) -> String {
        let url = format!("{}{}", base_url.trim_end_matches('/'), self.path);

        if self.query.is_empty() {
            url
        } else {
            let query_string: String = self
                .query
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}={}",
                        crate::apis::urlencode(k),
                        crate::apis::urlencode(v)
                    )
                })
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", url, query_string)
        }
    }
}
