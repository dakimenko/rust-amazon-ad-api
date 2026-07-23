use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::campaign_negative_keywords::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.spCampaignNegativeKeyword.v3+json";

pub async fn list_campaign_negative_keywords(
    configuration: &Configuration,
    request: Option<ListCampaignNegativeKeywordsRequest>,
) -> Result<ApiResponse<Vec<CampaignNegativeKeyword>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeKeywords/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_campaign_negative_keywords(
    configuration: &Configuration,
    body: Vec<CampaignNegativeKeyword>,
) -> Result<ApiResponse<Vec<CampaignNegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeKeywords", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_campaign_negative_keywords(
    configuration: &Configuration,
    body: Vec<CampaignNegativeKeyword>,
) -> Result<ApiResponse<Vec<CampaignNegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sp/campaignNegativeKeywords", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_campaign_negative_keywords(
    configuration: &Configuration,
    request: DeleteCampaignNegativeKeywordsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaignNegativeKeywords/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
