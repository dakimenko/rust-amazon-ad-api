use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::recommendations::ProductRecommendationRequest;
use crate::models::common::ApiResponse;

pub async fn create_ranked_keywords_recommendation(
    configuration: &Configuration,
    body: serde_json::Value,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/keywords/recommendations/ranked", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_ranked_keywords_recommendation(
    configuration: &Configuration,
    recommendation_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/keywords/recommendations/ranked/{}", configuration.base_path, recommendation_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_product_recommendations(
    configuration: &Configuration,
    body: ProductRecommendationRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/productRecommendations", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
