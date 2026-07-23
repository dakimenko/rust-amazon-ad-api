use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::negative_targeting::*;

const VERSION_HEADER: &str = "application/vnd.sdNegativeTargetingClause.v3+json";

pub async fn list_negative_targets(
    configuration: &Configuration,
    request: Option<SdListNegativeTargetsRequest>,
) -> Result<ApiResponse<Vec<SdNegativeTargetingClause>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/negativeTargets/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_negative_targets(
    configuration: &Configuration,
    body: Vec<SdNegativeTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/negativeTargets", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_negative_targets(
    configuration: &Configuration,
    body: Vec<SdNegativeTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sd/negativeTargets", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_negative_targets(
    configuration: &Configuration,
    request: SdDeleteNegativeTargetsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/negativeTargets/delete", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
