use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

use arc_swap::ArcSwapOption;

use crate::client::config::AmazonAdConfig;

#[derive(Debug, Serialize)]
pub struct LwaTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LwaTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct CachedToken {
    pub access_token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub profile_id: i64,
    pub country_code: Option<String>,
    pub currency_code: Option<String>,
    pub daily_budget: Option<f64>,
    pub timezone: Option<String>,
    pub account_info: Option<AccountInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub marketplace_string_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub name: Option<String>,
    pub valid_payment_method: Option<bool>,
}

/// OAuth2 client with lock-free token caching via `arc-swap`.
///
/// `cached_token` and `profile` use `ArcSwapOption` for zero-cost
/// concurrent reads across high-frequency API requests. A `Mutex`
/// is used only on the write path (token refresh), preventing
/// thundering-herd re-fetches.
#[derive(Clone)]
pub struct AuthClient {
    client: reqwest::Client,
    config: AmazonAdConfig,
    /// Lock-free read path: any thread can load the current token
    /// without blocking other readers or writers.
    cached_token: Arc<ArcSwapOption<CachedToken>>,
    /// Ensures only one task refreshes the token at a time.
    refresh_mutex: Arc<Mutex<()>>,
    /// Active profile for the API session.
    profile: Arc<ArcSwapOption<Profile>>,
}

impl AuthClient {
    pub fn new(config: AmazonAdConfig) -> Result<Self, anyhow::Error> {
        let user_agent = if let Some(ua) = &config.user_agent {
            ua.clone()
        } else {
            "amazon-ad-api-rs/0.1.0".to_string()
        };

        let mut client_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_sec.unwrap_or(30)))
            .user_agent(&user_agent);

        if let Some(proxy_url) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build()?;

        Ok(Self {
            client,
            config,
            cached_token: Arc::new(ArcSwapOption::empty()),
            refresh_mutex: Arc::new(Mutex::new(())),
            profile: Arc::new(ArcSwapOption::empty()),
        })
    }

    /// Clone this AuthClient with a different config while sharing the
    /// token cache and refresh mutex. This prevents redundant OAuth requests
    /// when switching profiles via `with_profile()`.
    pub fn clone_with_config(&self, new_config: AmazonAdConfig) -> Self {
        Self {
            client: self.client.clone(),
            config: new_config,
            cached_token: Arc::clone(&self.cached_token),
            refresh_mutex: Arc::clone(&self.refresh_mutex),
            profile: Arc::clone(&self.profile),
        }
    }

    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Check token validity without blocking — uses lock-free load.
    pub fn is_token_valid(&self) -> bool {
        if let Some(cached) = self.cached_token.load_full() {
            let current_time = Self::get_current_timestamp();
            return cached.expires_at > current_time + 300;
        }
        false
    }

    pub async fn get_access_token(&self) -> Result<String, anyhow::Error> {
        // Fast path: lock-free read with no contention
        if let Some(cached) = self.cached_token.load_full() {
            let current_time = Self::get_current_timestamp();
            if cached.expires_at > current_time + 300 {
                return Ok(cached.access_token.clone());
            }
        }

        // Slow path: serialize refresh to prevent thundering-herd
        let _guard = self.refresh_mutex.lock().await;

        // Double-check after acquiring mutex (another task may have refreshed)
        if let Some(cached) = self.cached_token.load_full() {
            let current_time = Self::get_current_timestamp();
            if cached.expires_at > current_time + 300 {
                return Ok(cached.access_token.clone());
            }
        }

        self.refresh_access_token().await
    }

    pub async fn refresh_access_token(&self) -> Result<String, anyhow::Error> {
        tracing::debug!("Refreshing access token...");

        let lwa_url = self.config.token_url();

        let request_body = LwaTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.expose_secret().to_string(),
            refresh_token: self.config.refresh_token.expose_secret().to_string(),
        };

        let response = self
            .client
            .post(&lwa_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: LwaTokenResponse = response.json().await?;

            tracing::debug!(
                "Received fresh access token (expires_in: {}s)",
                token_response.expires_in
            );

            let current_time = Self::get_current_timestamp();
            let expires_at = current_time + token_response.expires_in;

            // Atomic store — all concurrent readers immediately see the new token
            self.cached_token.store(Some(Arc::new(CachedToken {
                access_token: token_response.access_token.clone(),
                expires_at,
            })));

            Ok(token_response.access_token)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to get access token: {}",
                error_text
            ))
        }
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the active profile, if one has been set.
    pub fn get_profile(&self) -> Option<Arc<Profile>> {
        self.profile.load_full()
    }

    /// Get the active profile ID, if one has been set.
    pub fn get_profile_id(&self) -> Option<i64> {
        self.profile.load_full().as_ref().map(|p| p.profile_id)
    }

    /// Set the active profile atomically. Immediately visible to all readers.
    pub fn set_profile(&self, profile: Profile) {
        self.profile.store(Some(Arc::new(profile)));
    }

    /// Fetch available profiles using the provided access token.
    pub async fn fetch_profiles(&self, access_token: &str) -> Result<Vec<Profile>, anyhow::Error> {
        let base_url = if self.config.sandbox {
            self.config.region.sandbox_url()
        } else {
            self.config.region.base_url()
        };

        let url = format!("{}/v2/profiles", base_url);

        let response = self
            .client
            .get(&url)
            .header("Amazon-Advertising-API-ClientId", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let profiles: Vec<Profile> = response.json().await?;
            Ok(profiles)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch profiles: {}", error_text))
        }
    }
}
