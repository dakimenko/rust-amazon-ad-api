use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::snapshots::*;

pub async fn create_snapshot(
    configuration: &Configuration,
    body: SdSnapshotRequest,
) -> Result<ApiResponse<SdSnapshotResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/snapshots", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_snapshot_status(
    configuration: &Configuration,
    snapshot_id: &str,
) -> Result<ApiResponse<SdSnapshotStatusResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/sd/snapshots/{}", configuration.base_path, snapshot_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
