use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sp::bids::*;

pub async fn get_bid_recommendations(
    configuration: &Configuration,
    request: BidRecommendationsRequest,
) -> Result<ApiResponse<Vec<BidRecommendation>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sp/targets/bid/recommendations", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_theme_based_bid_recommendations(
    configuration: &Configuration,
    campaign_id: &str,
) -> Result<ApiResponse<Vec<ThemeBasedBidRecommendation>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!(
                "{}/sp/campaigns/{}/bid/recommendations",
                configuration.base_path, campaign_id
            ),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_keyword_bid_recommendations(
    configuration: &Configuration,
    request: BidRecommendationsRequest,
) -> Result<ApiResponse<Vec<BidRecommendation>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!(
                "{}/sp/keywords/bid/recommendations",
                configuration.base_path
            ),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
