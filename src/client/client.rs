use crate::apis::configuration::{Configuration, CustomClient};
use crate::client::{AuthClient, AmazonAdConfig, RateLimiter};

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
pub struct AmazonAdClient {
    pub(crate) http_client: reqwest::Client,
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

        let auth_client = AuthClient::new(config.clone())?;

        let rate_limiter = RateLimiter::new_with_safety_factor(
            config.rate_limit_factor.unwrap_or(1.10),
        );

        Ok(Self {
            http_client: client,
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
    pub async fn get_profile_id(&self) -> Option<i64> {
        self.auth_client.get_profile_id().await
    }

    /// Set the active profile.
    pub async fn set_profile(&self, profile: crate::client::auth::Profile) {
        self.auth_client.set_profile(profile).await
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
        let response = self
            .http_client
            .get(url)
            .send()
            .await?;

        if response.status().is_success() {
            let content = response.bytes().await?;
            tracing::info!(
                "Downloaded {} bytes from {}",
                content.len(),
                url
            );
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
    pub async fn create_configuration(
        &self,
    ) -> Result<Configuration, anyhow::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json; charset=utf-8".parse()?,
        );

        let base_url = self.get_base_url();
        let host = base_url.trim_start_matches("https://");
        headers.insert("host", host.parse()?);

        let access_token = self.get_access_token().await?;
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse()?,
        );

        headers.insert(
            "Amazon-Advertising-API-ClientId",
            self.config.client_id.parse()?,
        );

        // Only inject Scope header if a profile is set
        if let Some(ref profile_id) = self.config.profile_id {
            headers.insert(
                "Amazon-Advertising-API-Scope",
                profile_id.parse()?,
            );
        }

        let rate_limiter = std::sync::Arc::new(self.rate_limiter.clone());

        let configuration = Configuration {
            base_path: base_url,
            client: CustomClient::with_headers(
                self.http_client.clone(),
                self.config.retry_count.unwrap_or(0),
                headers,
            ),
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
        serde_json::from_str(s)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}: {}", e, s))
    }
}
