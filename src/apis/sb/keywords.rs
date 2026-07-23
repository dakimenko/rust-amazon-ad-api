use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::keywords::*;

pub async fn list_keywords(
    configuration: &Configuration,
    request: Option<SbListKeywordsRequest>,
) -> Result<ApiResponse<Vec<SbKeyword>>, Error<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut req = client
        .request(
            reqwest::Method::GET,
            format!("{}/sb/keywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
    if let Some(ref params) = request {
        req = req.query(params);
    }
    let req = req.build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_keywords(
    configuration: &Configuration,
    body: Vec<SbKeyword>,
) -> Result<ApiResponse<Vec<SbKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sb/keywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_keywords(
    configuration: &Configuration,
    body: Vec<SbKeyword>,
) -> Result<ApiResponse<Vec<SbKeywordResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sb/keywords", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_keyword(
    configuration: &Configuration,
    keyword_id: &str,
) -> Result<ApiResponse<SbKeyword>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/sb/keywords/{}", configuration.base_path, keyword_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_keyword(
    configuration: &Configuration,
    keyword_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::DELETE,
            format!("{}/sb/keywords/{}", configuration.base_path, keyword_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
