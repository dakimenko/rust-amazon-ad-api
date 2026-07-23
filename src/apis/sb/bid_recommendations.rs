use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;

pub async fn get_bid_recommendations(
    configuration: &Configuration,
    body: serde_json::Value,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/v2/hsa/bids/recommendations", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
