use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::targeting::*;

pub async fn list_targets(
    configuration: &Configuration,
    request: Option<SbListTargetsRequest>,
) -> Result<ApiResponse<Vec<SbTargetingClause>>, Error<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut req = client
        .request(
            reqwest::Method::GET,
            format!("{}/sb/targets", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
    if let Some(ref params) = request {
        req = req.query(params);
    }
    let req = req.build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_targets(
    configuration: &Configuration,
    body: Vec<SbTargetingClause>,
) -> Result<ApiResponse<Vec<SbTargetingClauseResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sb/targets", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_targets(
    configuration: &Configuration,
    body: Vec<SbTargetingClause>,
) -> Result<ApiResponse<Vec<SbTargetingClauseResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sb/targets", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_target(
    configuration: &Configuration,
    target_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::DELETE,
            format!("{}/sb/targets/{}", configuration.base_path, target_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
