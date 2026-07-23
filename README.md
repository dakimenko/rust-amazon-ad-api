# amazon-ad-api

[![Crates.io](https://img.shields.io/crates/v/amazon-ad-api)](https://crates.io/crates/amazon-ad-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Unofficial Rust client for the [Amazon Advertising API](https://advertising.amazon.com/API/docs/en-us).  
Full coverage of Sponsored Products, Sponsored Brands, Sponsored Display, and DSP.

Port of [`python-amazon-ad-api`](https://github.com/denisneuf/python-amazon-ad-api) v0.8.4.

## Quick Start

```rust
use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AmazonAdConfig::from_env()?;
    let client = AmazonAdClient::new(config)?;

    let response = client.sp_list_campaigns(None).await?;
    for campaign in response.payload {
        println!("{}: {:?}", campaign.name.as_deref().unwrap_or("?"), campaign.state);
    }
    Ok(())
}
```

## Configuration

### Environment Variables

```bash
export AD_API_CLIENT_ID="your-client-id"
export AD_API_CLIENT_SECRET="your-client-secret"
export AD_API_REFRESH_TOKEN="your-refresh-token"
export AD_API_PROFILE_ID="1234567890"
export AD_API_REGION="na"            # na | eu | fe (default: na)
export AD_API_SANDBOX="false"        # true | false
```

### TOML Config

Place at `~/.config/amazon-ad-api/credentials.toml` (Linux/macOS) or `%APPDATA%\amazon-ad-api\credentials.toml` (Windows):

```toml
client_id = "your-client-id"
client_secret = "your-client-secret"
refresh_token = "your-refresh-token"
profile_id = "1234567890"
region = "na"
sandbox = false
```

## Feature Flags

```toml
# Default: sp + sb + sd + client
amazon-ad-api = "0.1"

# Selective:
amazon-ad-api = { version = "0.1", features = ["sp", "sb"] }
```

| Feature  | Controls |
|----------|----------|
| `client` | HTTP client, auth, rate limiter (required for API calls) |
| `sp`     | Sponsored Products v3 |
| `sb`     | Sponsored Brands v4 |
| `sd`     | Sponsored Display |
| `dsp`    | DSP reports |
| `cross`  | Profiles, portfolios, reports v3, attribution, stream |

## Usage Examples

### Sponsored Products

```rust
use amazon_ad_api::models::sp::campaigns::*;

// Create campaign
let campaign = CampaignBuilder::default()
    .name("My Campaign")
    .state(CampaignState::Enabled)
    .targeting_type(TargetingType::Manual)
    .budget(DailyBudgetBuilder::default()
        .budget_type("DAILY".into()).budget(10.0).build()?)
    .build()?;
client.sp_create_campaigns(vec![campaign]).await?;

// List with filter
let filter = ListCampaignsRequestBuilder::default()
    .state_filter("enabled".into()).build()?;
client.sp_list_campaigns(Some(filter)).await?;
```

### Keywords

```rust
use amazon_ad_api::models::sp::keywords::*;

let kw = KeywordBuilder::default()
    .campaign_id("...")
    .ad_group_id("...")
    .keyword_text("running shoes")
    .match_type(MatchType::Exact)
    .bid(0.75)
    .state(KeywordState::Enabled)
    .build()?;
client.sp_create_keywords(vec![kw]).await?;
```

### Sponsored Brands v4

```rust
use amazon_ad_api::models::sb::campaigns::*;

let campaign = SbCampaignBuilder::default()
    .name("Brand Campaign")
    .state(SbCampaignState::Enabled)
    .budget(50.0)
    .creative_type(SbCreativeType::Video)
    .build()?;
client.sb_create_campaigns(vec![campaign]).await?;
```

### Reports

```rust
use amazon_ad_api::client::download::{download, DownloadFormat};

// Create, poll, download report
let req = SpReportRequestBuilder::default()
    .record_type("campaigns".into())
    .report_date("20240101".into())
    .metrics("impressions,clicks,cost".into())
    .build()?;
let resp = client.sp_create_report("campaigns", req).await?;

// Poll status...
// Download (auto gzip decompress, in-memory)
let result = download(client.get_http_client(), &url, DownloadFormat::StringValue).await?;
```

### Profile Switching

```rust
let eu_client = na_client.with_profile("europe-profile-id");
// Shares token cache — no redundant OAuth call
```

## Architecture

Three-layer design:
1. **Models** — strongly-typed serde structs with `derive_builder`
2. **Low-level APIs** — raw endpoint functions via `Configuration`
3. **Client APIs** — convenience methods on `AmazonAdClient`

Infrastructure:
- Adaptive token-bucket rate limiter (`x-ad-api-rate-limit-*` headers)
- OAuth2 LWA auth with `Arc`-shared token cache
- `reqwest-middleware` (retry, tracing, auth injection)
- In-memory downloads with gzip auto-decompression
- `async-stream` pagination generators

## Documentation

Run examples: `cargo run --example campaigns_list`

## Development

Task management convention:
- Active tasks → `tasks/`
- Completed tasks → `tasks/done_tasks/`
- See [`tasks/README.md`](tasks/README.md) for details.

## License

MIT
