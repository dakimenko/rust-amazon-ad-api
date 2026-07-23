use crate::apis::configuration::{Configuration, CustomClient};
use crate::client::{AmazonAdConfig, AuthClient, RateLimiter};

/// The main client for the Amazon Advertising API.
///
/// Provides authentication, rate limiting, and HTTP configuration for
/// all Advertising API endpoint calls.
///
/// # Example
/// ```no_run
/// use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};
///
/// # async fn example() -> Result<(), anyhow::Error> {
/// let config = AmazonAdConfig::from_env()?;
/// let client = AmazonAdClient::new(config)?;
/// let token = client.get_access_token().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct AmazonAdClient {
    pub(crate) http_client: reqwest::Client,
    pub(crate) custom_client: CustomClient,
    pub(crate) auth_client: AuthClient,
    pub(crate) config: AmazonAdConfig,
    pub(crate) rate_limiter: RateLimiter,
}

impl AmazonAdClient {
    /// Create a new Advertising API client with the given configuration.
    pub fn new(config: AmazonAdConfig) -> Result<Self, anyhow::Error> {
        let user_agent = if let Some(ua) = &config.user_agent {
            ua.clone()
        } else {
            Self::get_default_user_agent()
        };

        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_sec.unwrap_or(30),
            ))
            .user_agent(&user_agent);

        if let Some(proxy_url) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build()?;
        let custom_client = CustomClient::new(client.clone(), config.retry_count.unwrap_or(0));

        let auth_client = AuthClient::new(config.clone())?;

        let rate_limiter =
            RateLimiter::new_with_safety_factor(config.rate_limit_factor.unwrap_or(1.10));

        Ok(Self {
            http_client: client,
            custom_client,
            auth_client,
            config,
            rate_limiter,
        })
    }

    /// Create a shallow clone with a different profile ID.
    ///
    /// Useful for switching between advertising profiles without
    /// recreating the entire client (preserves token cache and HTTP pool).
    pub fn with_profile(&self, profile_id: impl Into<String>) -> Self {
        let mut new_config = self.config.clone();
        new_config.profile_id = Some(profile_id.into());

        Self {
            http_client: self.http_client.clone(),
            custom_client: self.custom_client.clone(),
            auth_client: self.auth_client.clone_with_config(new_config.clone()),
            config: new_config,
            rate_limiter: self.rate_limiter.clone(),
        }
    }

    /// Get a reference to the rate limiter.
    pub fn limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }

    /// Get the default user agent string.
    pub fn get_default_user_agent() -> String {
        let platform = format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH);
        format!(
            "amazon-ad-api/v{} (Language=Rust; Platform={})",
            env!("CARGO_PKG_VERSION"),
            platform
        )
    }

    /// Get the base URL for the API based on region and sandbox mode.
    pub fn get_base_url(&self) -> String {
        if self.config.sandbox {
            self.config.region.sandbox_url().to_string()
        } else {
            self.config.region.base_url().to_string()
        }
    }

    /// Get an access token from the auth client.
    pub async fn get_access_token(&self) -> Result<String, anyhow::Error> {
        self.auth_client.get_access_token().await
    }

    /// Get the active profile ID, if set.
    pub fn get_profile_id(&self) -> Option<i64> {
        self.auth_client.get_profile_id()
    }

    /// Set the active profile.
    pub fn set_profile(&self, profile: crate::client::auth::Profile) {
        self.auth_client.set_profile(profile)
    }

    /// Check if sandbox mode is enabled.
    pub fn is_sandbox(&self) -> bool {
        self.config.sandbox
    }

    /// Get the underlying HTTP client.
    pub fn get_http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Download bytes from a URL (side-effect-free, in-memory).
    pub async fn download_bytes(&self, url: &str) -> Result<bytes::Bytes, anyhow::Error> {
        let response = self.http_client.get(url).send().await?;

        if response.status().is_success() {
            let content = response.bytes().await?;
            tracing::info!("Downloaded {} bytes from {}", content.len(), url);
            Ok(content)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to download from {}: {} - {}",
                url,
                status,
                error_text
            ))
        }
    }

    /// Refresh the access token if needed.
    pub async fn refresh_access_token_if_needed(&self) -> Result<(), anyhow::Error> {
        if !self.auth_client.is_token_valid() {
            self.auth_client.refresh_access_token().await?;
        }
        Ok(())
    }

    /// Force refresh the access token.
    pub async fn force_refresh_token(&self) -> Result<(), anyhow::Error> {
        self.auth_client.refresh_access_token().await?;
        Ok(())
    }

    /// Create a `Configuration` for use with low-level API endpoint functions.
    ///
    /// This method fetches a fresh access token, builds the base URL,
    /// and injects all required Advertising API headers (ClientId,
    /// Authorization, Scope, Content-Type).
    pub async fn create_configuration(&self) -> Result<Configuration, anyhow::Error> {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, HOST};

        let mut headers = HeaderMap::with_capacity(5);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let base_url = self.get_base_url();
        let host_str = base_url.trim_start_matches("https://");
        if let Ok(val) = HeaderValue::from_str(host_str) {
            headers.insert(HOST, val);
        }

        let access_token = self.get_access_token().await?;
        let auth_val = format!("Bearer {}", access_token);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val)?);

        headers.insert(
            "Amazon-Advertising-API-ClientId",
            HeaderValue::from_str(&self.config.client_id)?,
        );

        // Only inject Scope header if a profile is set
        if let Some(ref profile_id) = self.config.profile_id {
            headers.insert(
                "Amazon-Advertising-API-Scope",
                HeaderValue::from_str(profile_id)?,
            );
        }

        let rate_limiter = std::sync::Arc::new(self.rate_limiter.clone());

        let configuration = Configuration {
            base_path: base_url,
            client: CustomClient {
                inner: self.custom_client.inner.clone(),
                headers,
            },
            user_agent: Some(
                self.config
                    .user_agent
                    .clone()
                    .unwrap_or_else(Self::get_default_user_agent),
            ),
            rate_limiter: Some(rate_limiter),
        };
        Ok(configuration)
    }

    /// Deserialize JSON from a string reference.
    pub fn from_json<'a, T>(s: &'a str) -> Result<T, anyhow::Error>
    where
        T: serde::Deserialize<'a>,
    {
        serde_json::from_str(s).map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}: {}", e, s))
    }
}
