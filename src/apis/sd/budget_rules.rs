use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sd::budget_rules::*;

pub async fn create_budget_rule(
    configuration: &Configuration,
    body: Vec<SdBudgetRule>,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sd/budgetRules", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn list_budget_rules(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<SdBudgetRule>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/sd/budgetRules", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .query(&[("nextToken", ""), ("maxResults", "100")])
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_budget_rule(
    configuration: &Configuration,
    rule_id: &str,
) -> Result<ApiResponse<SdBudgetRule>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/sd/budgetRules/{}", configuration.base_path, rule_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_budget_rule(
    configuration: &Configuration,
    body: Vec<SdBudgetRule>,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sd/budgetRules", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_budget_rule(
    configuration: &Configuration,
    rule_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::DELETE,
            format!("{}/sd/budgetRules/{}", configuration.base_path, rule_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
