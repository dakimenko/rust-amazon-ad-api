use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sd::brand_safety::*;
use crate::models::common::ApiResponse;

pub async fn get_deny_list(
    configuration: &Configuration,
) -> Result<ApiResponse<SdBrandSafetyDenyList>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/sd/brandSafety/denyList", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_deny_list(
    configuration: &Configuration,
    body: SdBrandSafetyDenyList,
) -> Result<ApiResponse<SdBrandSafetyDenyList>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/brandSafety/denyList", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
