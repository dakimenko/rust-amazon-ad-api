use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sb::negative_targeting::*;
use crate::models::common::ApiResponse;

pub async fn list_negative_targets(
    configuration: &Configuration,
    request: Option<SbListNegativeTargetsRequest>,
) -> Result<ApiResponse<Vec<SbNegativeTargetingClause>>, Error<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut req = client
        .request(reqwest::Method::GET, format!("{}/sb/negativeTargets", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
    if let Some(ref params) = request {
        req = req.query(params);
    }
    let req = req.build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_negative_targets(
    configuration: &Configuration,
    body: Vec<SbNegativeTargetingClause>,
) -> Result<ApiResponse<Vec<SbNegativeTargetingClauseResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sb/negativeTargets", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_negative_targets(
    configuration: &Configuration,
    body: Vec<SbNegativeTargetingClause>,
) -> Result<ApiResponse<Vec<SbNegativeTargetingClauseResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sb/negativeTargets", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_negative_target(
    configuration: &Configuration,
    target_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::DELETE, format!("{}/sb/negativeTargets/{}", configuration.base_path, target_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
