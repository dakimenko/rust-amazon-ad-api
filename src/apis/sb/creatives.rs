use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sb::creatives::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.sbcreativeresource.v4+json";

pub async fn list_creatives(
    configuration: &Configuration,
    request: Option<SbListCreativesRequest>,
) -> Result<ApiResponse<Vec<SbCreative>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sb/v4/creatives/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_creatives(
    configuration: &Configuration,
    body: Vec<SbCreative>,
) -> Result<ApiResponse<Vec<SbCreativeResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sb/v4/creatives", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_creatives(
    configuration: &Configuration,
    body: Vec<SbCreative>,
) -> Result<ApiResponse<Vec<SbCreativeResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sb/v4/creatives", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
