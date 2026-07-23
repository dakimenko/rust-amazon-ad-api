use serde::{Deserialize, Serialize};

/// Wrapper type for sensitive strings that redacts their content in Debug output.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(secret: impl Into<String>) -> Self {
        Self(secret.into())
    }

    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl serde::Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(SecretString)
    }
}

/// Configuration for the Amazon Advertising API client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmazonAdConfig {
    /// The client ID provided by Amazon.
    pub client_id: String,
    /// The client secret provided by Amazon.
    pub client_secret: SecretString,
    /// The refresh token for obtaining access tokens.
    pub refresh_token: SecretString,
    /// The active advertising profile ID (None for DSP).
    pub profile_id: Option<String>,
    /// The advertising API region.
    #[serde(default)]
    pub region: Region,
    /// Whether to use the sandbox environment.
    #[serde(default)]
    pub sandbox: bool,
    /// Custom token URL override.
    pub token_url: Option<String>,
    /// Request timeout in seconds. Defaults to 30s.
    pub timeout_sec: Option<u64>,
    /// Rate limit safety factor. Defaults to 1.1.
    pub rate_limit_factor: Option<f64>,
    /// Optional proxy URL.
    pub proxy: Option<String>,
    /// Number of retry attempts for 429/5xx. None/0 = no retries.
    pub retry_count: Option<usize>,
    /// Custom user agent string.
    pub user_agent: Option<String>,
}

/// Advertising API region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Region {
    #[serde(rename = "na")]
    #[default]
    NorthAmerica,
    #[serde(rename = "eu")]
    Europe,
    #[serde(rename = "fe")]
    FarEast,
}

impl Region {
    pub const fn base_url(&self) -> &'static str {
        match self {
            Region::NorthAmerica => "https://advertising-api.amazon.com",
            Region::Europe => "https://advertising-api-eu.amazon.com",
            Region::FarEast => "https://advertising-api-fe.amazon.com",
        }
    }

    pub const fn sandbox_url(&self) -> &'static str {
        "https://advertising-api-test.amazon.com"
    }
}

impl std::str::FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "na" | "northamerica" | "north-america" => Ok(Region::NorthAmerica),
            "eu" | "europe" => Ok(Region::Europe),
            "fe" | "fareast" | "far-east" => Ok(Region::FarEast),
            _ => Err(anyhow::anyhow!("Invalid region string: {}", s)),
        }
    }
}

impl AmazonAdConfig {
    /// Create config from environment variables.
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let client_id = std::env::var("AD_API_CLIENT_ID")
            .map_err(|_| anyhow::anyhow!("AD_API_CLIENT_ID env var is not set"))?;
        let client_secret = std::env::var("AD_API_CLIENT_SECRET")
            .map_err(|_| anyhow::anyhow!("AD_API_CLIENT_SECRET env var is not set"))?;
        let refresh_token = std::env::var("AD_API_REFRESH_TOKEN")
            .map_err(|_| anyhow::anyhow!("AD_API_REFRESH_TOKEN env var is not set"))?;
        let profile_id = std::env::var("AD_API_PROFILE_ID").ok();
        let region = std::env::var("AD_API_REGION")
            .ok()
            .map(|r| r.parse::<Region>())
            .transpose()?
            .unwrap_or_default();
        let sandbox = std::env::var("AD_API_SANDBOX")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Ok(Self {
            client_id,
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
            profile_id,
            region,
            sandbox,
            token_url: None,
            user_agent: None,
            timeout_sec: Some(30),
            rate_limit_factor: None,
            proxy: None,
            retry_count: None,
        })
    }

    /// Create config from a TOML file.
    pub fn from_file(path: &str) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Create config from the default credentials file.
    /// On Unix: `~/.config/amazon-ad-api/credentials.toml`
    /// On Windows: `%APPDATA%\amazon-ad-api\credentials.toml`
    pub fn from_default_file() -> Result<Self, anyhow::Error> {
        let path = if cfg!(windows) {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
            format!("{}\\amazon-ad-api\\credentials.toml", appdata)
        } else {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.config/amazon-ad-api/credentials.toml", home)
        };
        Self::from_file(&path)
    }

    /// Get the OAuth2 token URL for the configured region.
    pub fn token_url(&self) -> String {
        match self.region {
            Region::NorthAmerica => "https://api.amazon.com/auth/o2/token".to_string(),
            Region::Europe => "https://api.amazon.co.uk/auth/o2/token".to_string(),
            Region::FarEast => "https://api.amazon.co.jp/auth/o2/token".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_region_from_str_valid() {
        assert_eq!(Region::from_str("na").unwrap(), Region::NorthAmerica);
        assert_eq!(Region::from_str("eu").unwrap(), Region::Europe);
        assert_eq!(Region::from_str("fe").unwrap(), Region::FarEast);
        assert_eq!(
            "northamerica".parse::<Region>().unwrap(),
            Region::NorthAmerica
        );
        assert_eq!("fareast".parse::<Region>().unwrap(), Region::FarEast);
    }

    #[test]
    fn test_region_from_str_invalid() {
        assert!(Region::from_str("xx").is_err());
        assert!(Region::from_str("").is_err());
    }

    #[test]
    fn test_region_base_urls() {
        assert!(Region::NorthAmerica.base_url().contains("advertising-api"));
        assert!(Region::Europe.base_url().contains("eu"));
        assert!(Region::FarEast.base_url().contains("fe"));
    }

    #[test]
    fn test_region_sandbox_url() {
        assert!(Region::NorthAmerica.sandbox_url().contains("test"));
        assert!(Region::Europe.sandbox_url().contains("test"));
    }

    #[test]
    fn test_config_from_env_missing() {
        // Without env vars set, this should fail
        std::env::remove_var("AD_API_CLIENT_ID");
        std::env::remove_var("AD_API_CLIENT_SECRET");
        std::env::remove_var("AD_API_REFRESH_TOKEN");
        assert!(AmazonAdConfig::from_env().is_err());
    }

    #[test]
    fn test_token_urls() {
        let config = AmazonAdConfig {
            client_id: "test".into(),
            client_secret: "test".into(),
            refresh_token: "test".into(),
            profile_id: None,
            region: Region::NorthAmerica,
            sandbox: false,
            token_url: None,
            user_agent: None,
            timeout_sec: Some(30),
            rate_limit_factor: None,
            proxy: None,
            retry_count: None,
        };
        assert!(config.token_url().contains("amazon.com"));
    }

    #[test]
    fn test_secret_string_redaction() {
        let secret = SecretString::new("my-super-secret-token");
        assert_eq!(format!("{:?}", secret), "[REDACTED]");
        assert_eq!(secret.expose_secret(), "my-super-secret-token");
    }
}
