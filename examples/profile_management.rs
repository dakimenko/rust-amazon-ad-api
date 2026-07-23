//! List and select advertising profiles.
//!
//! Usage:
//!   export AD_API_CLIENT_ID=...
//!   export AD_API_CLIENT_SECRET=...
//!   export AD_API_REFRESH_TOKEN=...
//!   cargo run --example profile_management

use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // Load config without a profile_id — we'll select one at runtime
    let config = AmazonAdConfig::from_env()?;
    println!("Obtaining access token...");
    let client = AmazonAdClient::new(config.clone())?;
    let token = client.get_access_token().await?;
    println!("Token obtained ({} chars)", token.len());

    // List available profiles using the raw HTTP client
    let base_url = if config.sandbox {
        "https://advertising-api.amazon.com"
    } else {
        config.region.base_url()
    };
    let url = format!("{}/v2/profiles", base_url);

    println!("Fetching profiles...");
    let response = client
        .get_http_client()
        .get(&url)
        .header("Amazon-Advertising-API-ClientId", &config.client_id)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    let profiles: Vec<serde_json::Value> = response.json().await?;
    println!("Available profiles:\n");

    for p in &profiles {
        let profile_id = p["profileId"].as_i64().unwrap_or(0);
        let country = p["countryCode"].as_str().unwrap_or("???");
        let currency = p["currencyCode"].as_str().unwrap_or("???");
        let timezone = p["timezone"].as_str().unwrap_or("???");
        let name = p["accountInfo"]["name"]
            .as_str()
            .unwrap_or("unnamed");

        println!(
            "  ID: {profile_id:<12} | {country:<3} | {currency:<4} | {timezone:<30} | {name}"
        );
    }

    // Select the first profile
    if let Some(first) = profiles.first() {
        if let Some(id) = first["profileId"].as_i64() {
            println!(
                "\nWould set profile_id={id} in config to use this profile."
            );
            println!("Example: config.profile_id = Some(\"{id}\".into());");
        }
    }

    Ok(())
}
