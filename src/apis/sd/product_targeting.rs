use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sd::targeting::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.sdTargetingClause.v3+json";

pub async fn list_targets(
    configuration: &Configuration,
    request: Option<SdListTargetsRequest>,
) -> Result<ApiResponse<Vec<SdTargetingClause>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/targets/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_targets(
    configuration: &Configuration,
    body: Vec<SdTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/targets", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_targets(
    configuration: &Configuration,
    body: Vec<SdTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sd/targets", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_targets(
    configuration: &Configuration,
    request: SdDeleteTargetsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/targets/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
