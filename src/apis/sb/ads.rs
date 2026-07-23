use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sb::ads::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.sbadresource.v4+json";

pub async fn list_ads(
    configuration: &Configuration,
    request: Option<SbListAdsRequest>,
) -> Result<ApiResponse<Vec<SbAd>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sb/v4/ads/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_ads(
    configuration: &Configuration,
    body: Vec<SbAd>,
) -> Result<ApiResponse<Vec<SbAdResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sb/v4/ads", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_ads(
    configuration: &Configuration,
    body: Vec<SbAd>,
) -> Result<ApiResponse<Vec<SbAdResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sb/v4/ads", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
