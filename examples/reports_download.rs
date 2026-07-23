//! Create a Sponsored Products campaign report, poll for completion,
//! download, decompress, and save as CSV.
//!
//! Usage:
//!   export AD_API_CLIENT_ID=...
//!   export AD_API_CLIENT_SECRET=...
//!   export AD_API_REFRESH_TOKEN=...
//!   export AD_API_PROFILE_ID=...
//!   cargo run --example reports_download

use amazon_ad_api::client::download::{download, DownloadFormat, DownloadResult};
use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};
use amazon_ad_api::models::sp::reports::SpReportRequestBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let config = AmazonAdConfig::from_env()?;
    let client = AmazonAdClient::new(config)?;

    // Step 1: Create report
    println!("Creating campaign report...");
    let req = SpReportRequestBuilder::default()
        .record_type("campaigns".to_string())
        .report_date("20240101".to_string())
        .metrics("impressions,clicks,cost,spend,sales".to_string())
        .build()?;

    let create_resp = client.sp_create_report("campaigns", req).await?;
    let report_id = create_resp
        .payload
        .report_id
        .clone()
        .ok_or_else(|| anyhow::anyhow!("No report_id in response"))?;
    println!("Report created: {report_id}");

    // Step 2: Poll for completion
    println!("Waiting for report to complete...");
    let location = loop {
        let status_resp = client.sp_get_report_status(&report_id).await?;
        let status = status_resp.payload.status.as_deref().unwrap_or("UNKNOWN");

        println!("  Status: {status}");
        if status == "COMPLETED" {
            break status_resp
                .payload
                .location
                .or(status_resp.payload.url)
                .ok_or_else(|| anyhow::anyhow!("No download URL in completed report"))?;
        }
        if status == "FAILURE" {
            anyhow::bail!(
                "Report failed: {}",
                status_resp
                    .payload
                    .status_details
                    .as_deref()
                    .unwrap_or("no details")
            );
        }
        tokio::time::sleep(Duration::from_secs(15)).await;
    };

    // Step 3: Download and decompress (in-memory, no disk writes)
    println!("Downloading and decompressing report...");
    let result = download(
        client.get_http_client(),
        &location,
        DownloadFormat::StringValue,
    )
    .await?;

    // Step 4: Save to file
    match result {
        DownloadResult::String(csv) => {
            std::fs::write("report.csv", &csv)?;
            let lines = csv.lines().count();
            println!("Report saved to report.csv ({lines} lines)");
            // Print first 3 lines
            for line in csv.lines().take(3) {
                println!("  {line}");
            }
        }
        _ => anyhow::bail!("Unexpected download format"),
    }

    Ok(())
}
