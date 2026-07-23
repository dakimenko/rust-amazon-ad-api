use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::campaign_negative_targets::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.spCampaignNegativeTargetingClause.v3+json";

pub async fn list_campaign_negative_targets(
    configuration: &Configuration,
    request: Option<ListCampaignNegativeTargetsRequest>,
) -> Result<ApiResponse<Vec<CampaignNegativeTargetingClause>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeTargets/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_campaign_negative_targets(
    configuration: &Configuration,
    body: Vec<CampaignNegativeTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeTargets", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_campaign_negative_targets(
    configuration: &Configuration,
    body: Vec<CampaignNegativeTargetingClause>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sp/campaignNegativeTargets", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_campaign_negative_targets(
    configuration: &Configuration,
    request: DeleteCampaignNegativeTargetsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeTargets/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
