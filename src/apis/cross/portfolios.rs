use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::cross::portfolios::*;

pub async fn list_portfolios(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<Portfolio>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/v2/portfolios", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_portfolios(
    configuration: &Configuration,
    body: Vec<Portfolio>,
) -> Result<ApiResponse<Vec<Portfolio>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/v2/portfolios", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_portfolios(
    configuration: &Configuration,
    body: Vec<Portfolio>,
) -> Result<ApiResponse<Vec<Portfolio>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/v2/portfolios", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_portfolio(
    configuration: &Configuration,
    portfolio_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::DELETE,
            format!("{}/v2/portfolios/{}", configuration.base_path, portfolio_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    let resp = configuration.client.execute(req).await?;
    let status = resp.status();
    let content = resp.text().await?;
    if status.is_success() {
        let payload: serde_json::Value = if content.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(&content)?
        };
        Ok(ApiResponse {
            payload,
            headers: Default::default(),
            next_token: None,
            rate_limit: None,
        })
    } else {
        Err(Error::ResponseError(crate::apis::ResponseContent {
            status,
            content,
            entity: None,
        }))
    }
}
