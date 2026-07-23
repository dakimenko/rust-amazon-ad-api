use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::campaigns::*;

const VERSION_HEADER: &str = "application/vnd.sbcampaignresource.v4+json";

pub async fn list_campaigns(
    configuration: &Configuration,
    request: Option<SbListCampaignsRequest>,
) -> Result<ApiResponse<Vec<SbCampaign>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sb/v4/campaigns/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_campaigns(
    configuration: &Configuration,
    body: Vec<SbCampaign>,
) -> Result<ApiResponse<Vec<SbCampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sb/v4/campaigns", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_campaigns(
    configuration: &Configuration,
    body: Vec<SbCampaign>,
) -> Result<ApiResponse<Vec<SbCampaignResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sb/v4/campaigns", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
