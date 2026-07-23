use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::ad_groups::*;

const VERSION_HEADER: &str = "application/vnd.sdAdGroup.v3+json";

pub async fn list_ad_groups(
    configuration: &Configuration,
    request: Option<SdListAdGroupsRequest>,
) -> Result<ApiResponse<Vec<SdAdGroup>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/adGroups/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_ad_groups(
    configuration: &Configuration,
    body: Vec<SdAdGroup>,
) -> Result<ApiResponse<Vec<SdAdGroupResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/adGroups", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_ad_groups(
    configuration: &Configuration,
    body: Vec<SdAdGroup>,
) -> Result<ApiResponse<Vec<SdAdGroupResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sd/adGroups", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_ad_groups(
    configuration: &Configuration,
    request: SdDeleteAdGroupsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/adGroups/delete", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
