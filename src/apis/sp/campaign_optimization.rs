use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sp::optimization::*;
use crate::models::common::ApiResponse;

pub async fn check_optimization_eligibility(
    configuration: &Configuration,
    body: OptimizationEligibilityRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/rules/eligibility", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_optimization_rule(
    configuration: &Configuration,
    body: Vec<CampaignOptimizationRule>,
) -> Result<ApiResponse<Vec<serde_json::Value>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sp/rules/campaignOptimization", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn list_optimization_rules(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<CampaignOptimizationRule>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/rules/campaignOptimization", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_optimization_rule(
    configuration: &Configuration,
    rule_id: &str,
) -> Result<ApiResponse<CampaignOptimizationRule>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/rules/campaignOptimization/{}", configuration.base_path, rule_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_optimization_rule(
    configuration: &Configuration,
    rule_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::DELETE, format!("{}/sp/rules/campaignOptimization/{}", configuration.base_path, rule_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_consolidated_recommendations(
    configuration: &Configuration,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sp/campaigns/recommendations/consolidated", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
