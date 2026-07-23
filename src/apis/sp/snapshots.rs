use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::snapshots::*;
use crate::models::common::ApiResponse;

pub async fn create_snapshot(
    configuration: &Configuration,
    body: SnapshotRequest,
) -> Result<ApiResponse<SnapshotResponse>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/snapshots", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_snapshot_status(
    configuration: &Configuration,
    snapshot_id: &str,
) -> Result<ApiResponse<SnapshotStatusResponse>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/snapshots/{}", configuration.base_path, snapshot_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
