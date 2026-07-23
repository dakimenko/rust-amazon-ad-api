use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::campaigns::*;

const VERSION_HEADER: &str = "application/vnd.sdCampaign.v3+json";

pub async fn list_campaigns(
    configuration: &Configuration,
    request: Option<SdListCampaignsRequest>,
) -> Result<ApiResponse<Vec<SdCampaign>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/campaigns/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_campaigns(
    configuration: &Configuration,
    body: Vec<SdCampaign>,
) -> Result<ApiResponse<Vec<SdCampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/campaigns", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_campaigns(
    configuration: &Configuration,
    body: Vec<SdCampaign>,
) -> Result<ApiResponse<Vec<SdCampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sd/campaigns", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_campaigns(
    configuration: &Configuration,
    request: SdDeleteCampaignsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/campaigns/delete", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
