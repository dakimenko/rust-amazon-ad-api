use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::cross::profiles::*;

pub async fn list_profiles(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<Profile>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/v2/profiles", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_profile(
    configuration: &Configuration,
    profile_id: &str,
) -> Result<ApiResponse<Profile>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/v2/profiles/{}", configuration.base_path, profile_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn register_profile(
    configuration: &Configuration,
    body: RegisterProfileRequest,
) -> Result<ApiResponse<Profile>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/v2/profiles/register", configuration.base_path),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
