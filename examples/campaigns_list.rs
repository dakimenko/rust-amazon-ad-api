//! List Sponsored Products campaigns.
//!
//! Usage:
//!   export AD_API_CLIENT_ID=...
//!   export AD_API_CLIENT_SECRET=...
//!   export AD_API_REFRESH_TOKEN=...
//!   export AD_API_PROFILE_ID=...
//!   cargo run --example campaigns_list

use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};
use amazon_ad_api::models::sp::campaigns::ListCampaignsRequestBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let config = AmazonAdConfig::from_env()?;
    let client = AmazonAdClient::new(config)?;

    println!("Listing Sponsored Products campaigns...\n");

    // List all campaigns (no filter)
    let response = client.sp_list_campaigns(None).await?;
    println!("Found {} campaigns:\n", response.payload.len());

    for campaign in &response.payload {
        let name = campaign.name.as_deref().unwrap_or("unnamed");
        let state = campaign
            .state
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "unknown".to_string());
        let budget = campaign.budget.as_ref().map(|b| b.budget).unwrap_or(0.0);
        let id = campaign.campaign_id.as_deref().unwrap_or("???");

        println!("  {name:<40} | {state:<10} | ${budget:>8.2} | {id}",);
    }

    // List only enabled campaigns with a filter
    println!("\n--- Filtered: enabled only ---\n");
    let filter = ListCampaignsRequestBuilder::default()
        .state_filter("enabled".to_string())
        .build()?;
    let filtered = client.sp_list_campaigns(Some(filter)).await?;

    for campaign in &filtered.payload {
        let name = campaign.name.as_deref().unwrap_or("unnamed");
        println!("  {name}");
    }

    Ok(())
}
