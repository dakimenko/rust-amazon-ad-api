use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing for visibility into rate limiting logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Configure the client with a custom safety factor.
    // A safety factor > 1.0 (e.g., 1.10) ensures we wait slightly longer than
    // strictly necessary, preventing HTTP 429 Too Many Requests errors.
    let mut config = AmazonAdConfig::from_env()?;
    config.rate_limit_factor = Some(1.20); // 20% safety margin

    let client = AmazonAdClient::new(config)?;

    let limiter = client.limiter();

    println!("Initial Rate Limiter Safety Factor: {}", limiter.get_safety_factor());

    // You can dynamically update limits if you know them from documentation,
    // though the SDK normally updates them automatically from response headers.
    let endpoint_id = "sp/campaigns";
    let default_rate = 10.0; // 10 requests per second
    let default_burst = 50;  // Burst of 50 requests

    println!("Simulating 5 requests to {}...", endpoint_id);

    for i in 1..=5 {
        // Wait for a token. This will block if the bucket is empty.
        let guard = limiter.wait(endpoint_id, default_rate, default_burst).await;
        
        println!("Request {} acquired token.", i);
        
        // Simulating the actual API call
        sleep(Duration::from_millis(50)).await;

        // The guard is automatically dropped here, which records the response time
        // to maintain the correct minimum interval between requests.
        drop(guard);
    }

    // Inspect the internal status of the buckets
    let status = limiter.get_token_status().await;
    for (endpoint, (tokens, rate, burst)) in status {
        println!(
            "Endpoint: {}, Tokens: {:.2}, Rate: {}, Burst: {}",
            endpoint, tokens, rate, burst
        );
    }

    Ok(())
}
