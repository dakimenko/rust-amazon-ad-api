use reqwest::header::HeaderMap;
use std::sync::Arc;

/// Configuration wrapper for low-level API calls.
///
/// Holds the base URL, user agent, HTTP client with middleware
/// (retry, tracing, auth header injection), and optional rate limiter.
#[derive(Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: CustomClient,
    pub rate_limiter: Option<Arc<crate::client::RateLimiter>>,
}

impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Configuration")
            .field("base_path", &self.base_path)
            .field("user_agent", &self.user_agent)
            .field("rate_limiter", &self.rate_limiter.as_ref().map(|_| "<RateLimiter>"))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CustomClient {
    pub(crate) inner: reqwest_middleware::ClientWithMiddleware,
    pub(crate) headers: HeaderMap,
}

impl CustomClient {
    pub fn new(client: reqwest::Client, retry_count: usize) -> Self {
        Self::with_headers(client, retry_count, HeaderMap::new())
    }

    pub fn with_headers(
        client: reqwest::Client,
        retry_count: usize,
        headers: HeaderMap,
    ) -> Self {
        let mut builder = reqwest_middleware::ClientBuilder::new(client);

        builder = builder.with(reqwest_tracing::TracingMiddleware::default());

        if retry_count > 0 {
            let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(retry_count as u32);
            builder = builder
                .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                    retry_policy,
                ));
        }

        let inner = builder.build();
        Self { inner, headers }
    }

    pub fn request<U: reqwest::IntoUrl>(
        &self,
        method: reqwest::Method,
        url: U,
    ) -> reqwest_middleware::RequestBuilder {
        self.inner.request(method, url)
    }

    pub async fn execute(
        &self,
        mut request: reqwest::Request,
    ) -> Result<reqwest::Response, reqwest_middleware::Error> {
        tracing::debug!("Executing request to: {}", request.url());
        tracing::debug!("Executing request method: {}", request.method());

        let req_headers = request.headers_mut();
        for (key, value) in self.headers.iter() {
            req_headers.insert(key.clone(), value.clone());
        }

        let result = self.inner.execute(request).await;

        if let Ok(resp) = &result {
            tracing::debug!("Response status: {:?}", resp.status());
        } else if let Err(e) = &result {
            tracing::error!("Request execution failed: {:?}", e);
        }

        result
    }
}

impl std::ops::Deref for CustomClient {
    type Target = reqwest_middleware::ClientWithMiddleware;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
