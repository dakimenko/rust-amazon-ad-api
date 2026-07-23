use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::negative_keywords::*;

pub async fn list_negative_keywords(
    configuration: &Configuration,
    request: Option<SbListNegativeKeywordsRequest>,
) -> Result<ApiResponse<Vec<SbNegativeKeyword>>, Error<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut req = client
        .request(
            reqwest::Method::GET,
            format!("{}/sb/negativeKeywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
    if let Some(ref params) = request {
        req = req.query(params);
    }
    let req = req.build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_negative_keywords(
    configuration: &Configuration,
    body: Vec<SbNegativeKeyword>,
) -> Result<ApiResponse<Vec<SbNegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sb/negativeKeywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_negative_keywords(
    configuration: &Configuration,
    body: Vec<SbNegativeKeyword>,
) -> Result<ApiResponse<Vec<SbNegativeKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sb/negativeKeywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_negative_keyword(
    configuration: &Configuration,
    keyword_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::DELETE,
            format!(
                "{}/sb/negativeKeywords/{}",
                configuration.base_path, keyword_id
            ),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
