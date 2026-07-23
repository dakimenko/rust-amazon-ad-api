use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};

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

#[derive(Clone)]
pub struct AuthClient {
    client: reqwest::Client,
    config: AmazonAdConfig,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
    refresh_mutex: Arc<Mutex<()>>,
    profile: Arc<RwLock<Option<Profile>>>,
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
            cached_token: Arc::new(RwLock::new(None)),
            refresh_mutex: Arc::new(Mutex::new(())),
            profile: Arc::new(RwLock::new(None)),
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

    pub fn is_token_valid(&self) -> bool {
        if let Ok(cached_lock) = self.cached_token.try_read() {
            if let Some(ref cached) = *cached_lock {
                let current_time = Self::get_current_timestamp();
                let buffer_time = 300;
                return cached.expires_at > current_time + buffer_time;
            }
        }
        false
    }

    pub async fn get_access_token(&self) -> Result<String, anyhow::Error> {
        // Fast path: check valid cached token
        {
            let cached_lock = self.cached_token.read().await;
            if let Some(ref cached) = *cached_lock {
                let current_time = Self::get_current_timestamp();
                let buffer_time = 300;
                if cached.expires_at > current_time + buffer_time {
                    return Ok(cached.access_token.clone());
                }
            }
        }

        // Slow path: synchronize refresh
        let _guard = self.refresh_mutex.lock().await;

        // Double-check after acquiring mutex
        {
            let cached_lock = self.cached_token.read().await;
            if let Some(ref cached) = *cached_lock {
                let current_time = Self::get_current_timestamp();
                let buffer_time = 300;
                if cached.expires_at > current_time + buffer_time {
                    return Ok(cached.access_token.clone());
                }
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

            let mut cached_lock = self.cached_token.write().await;
            *cached_lock = Some(CachedToken {
                access_token: token_response.access_token.clone(),
                expires_at,
            });

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
    pub async fn get_profile(&self) -> Option<Profile> {
        self.profile.read().await.clone()
    }

    /// Get the active profile ID, if one has been set.
    pub async fn get_profile_id(&self) -> Option<i64> {
        self.profile.read().await.as_ref().map(|p| p.profile_id)
    }

    /// Set the active profile. Used after selecting from the profiles list.
    pub async fn set_profile(&self, profile: Profile) {
        let mut p = self.profile.write().await;
        *p = Some(profile);
    }

    /// Fetch available profiles using the provided access token.
    /// Requires the token to already be obtained.
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
