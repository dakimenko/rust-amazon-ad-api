use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::ad_groups::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.spAdGroup.v3+json";

pub async fn list_ad_groups(
    configuration: &Configuration,
    request: Option<ListAdGroupsRequest>,
) -> Result<ApiResponse<Vec<AdGroup>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/adGroups/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_ad_groups(
    configuration: &Configuration,
    body: Vec<AdGroup>,
) -> Result<ApiResponse<Vec<AdGroupResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/adGroups", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_ad_groups(
    configuration: &Configuration,
    body: Vec<AdGroup>,
) -> Result<ApiResponse<Vec<AdGroupResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sp/adGroups", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_ad_groups(
    configuration: &Configuration,
    request: DeleteAdGroupsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/adGroups/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
