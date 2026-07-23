use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;

pub async fn get_initial_budget_recommendation(
    configuration: &Configuration,
    body: serde_json::Value,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/campaigns/initialBudgetRecommendation", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_budget_recommendations(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/campaigns/budgetRecommendations", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_budget_rules_recommendations(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/budgetRules/recommendations", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_budget_usage(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/campaigns/budgetUsage", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
