use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::campaigns::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.spCampaign.v3+json";

pub async fn list_campaigns(
    configuration: &Configuration,
    request: Option<ListCampaignsRequest>,
) -> Result<ApiResponse<Vec<Campaign>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaigns/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_campaigns(
    configuration: &Configuration,
    body: Vec<Campaign>,
) -> Result<ApiResponse<Vec<CampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaigns", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_campaigns(
    configuration: &Configuration,
    body: Vec<Campaign>,
) -> Result<ApiResponse<Vec<CampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sp/campaigns", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_campaigns(
    configuration: &Configuration,
    request: DeleteCampaignsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaigns/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
