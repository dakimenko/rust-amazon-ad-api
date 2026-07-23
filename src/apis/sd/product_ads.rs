use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sd::product_ads::*;
use crate::models::common::ApiResponse;

const VERSION_HEADER: &str = "application/vnd.sdProductAd.v3+json";

pub async fn list_product_ads(
    configuration: &Configuration,
    request: Option<SdListProductAdsRequest>,
) -> Result<ApiResponse<Vec<SdProductAd>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/productAds/list", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn create_product_ads(
    configuration: &Configuration,
    body: Vec<SdProductAd>,
) -> Result<ApiResponse<Vec<SdProductAdResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/productAds", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn update_product_ads(
    configuration: &Configuration,
    body: Vec<SdProductAd>,
) -> Result<ApiResponse<Vec<SdProductAdResponse>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::PUT, format!("{}/sd/productAds", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_product_ads(
    configuration: &Configuration,
    request: SdDeleteProductAdsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/sd/productAds/delete", configuration.base_path))
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
