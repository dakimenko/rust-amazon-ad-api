use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::creatives::*;

const VERSION_HEADER: &str = "application/vnd.sdCreative.v3+json";

pub async fn list_creatives(
    configuration: &Configuration,
    request: Option<SdListCreativesRequest>,
) -> Result<ApiResponse<Vec<SdCreative>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/creatives/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_creatives(
    configuration: &Configuration,
    body: Vec<SdCreative>,
) -> Result<ApiResponse<Vec<SdCreativeResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/creatives", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_creatives(
    configuration: &Configuration,
    body: Vec<SdCreative>,
) -> Result<ApiResponse<Vec<SdCreativeResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sd/creatives", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
