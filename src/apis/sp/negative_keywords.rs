use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::negative_keywords::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.spNegativeKeyword.v3+json";

pub async fn list_negative_keywords(
    configuration: &Configuration,
    request: Option<ListNegativeKeywordsRequest>,
) -> Result<ApiResponse<Vec<NegativeKeyword>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/negativeKeywords/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_negative_keywords(
    configuration: &Configuration,
    body: Vec<NegativeKeyword>,
) -> Result<ApiResponse<Vec<NegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/negativeKeywords", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_negative_keywords(
    configuration: &Configuration,
    body: Vec<NegativeKeyword>,
) -> Result<ApiResponse<Vec<NegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sp/negativeKeywords", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_negative_keywords(
    configuration: &Configuration,
    request: DeleteNegativeKeywordsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/negativeKeywords/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
