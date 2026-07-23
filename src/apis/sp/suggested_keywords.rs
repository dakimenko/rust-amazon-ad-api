use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::recommendations::*;
use crate::models::common::ApiResponse;

pub async fn get_suggested_keywords(
    configuration: &Configuration,
    request: SuggestedKeywordsRequest,
) -> Result<ApiResponse<Vec<SuggestedKeyword>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/adGroups/suggestedKeywords", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_suggested_keywords_extended(
    configuration: &Configuration,
    request: SuggestedKeywordsRequest,
) -> Result<ApiResponse<Vec<SuggestedKeyword>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/adGroups/suggestedKeywords/extended", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
