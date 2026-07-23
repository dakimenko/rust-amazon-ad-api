use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::media::*;

pub async fn upload_media(
    configuration: &Configuration,
    form: reqwest::multipart::Form,
) -> Result<ApiResponse<SbMediaUploadResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/v2/hsa/media", configuration.base_path),
        )
        .header("Accept", "application/json")
        .multipart(form)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_media_status(
    configuration: &Configuration,
    media_id: &str,
) -> Result<ApiResponse<SbMediaUploadResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/v2/hsa/media/{}", configuration.base_path, media_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn complete_media(
    configuration: &Configuration,
    media_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!(
                "{}/v2/hsa/media/{}/complete",
                configuration.base_path, media_id
            ),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
