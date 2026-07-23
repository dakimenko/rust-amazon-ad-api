use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sp::product_ads::*;

const VERSION_HEADER: &str = "application/vnd.spProductAd.v3+json";

pub async fn list_product_ads(
    configuration: &Configuration,
    request: Option<ListProductAdsRequest>,
) -> Result<ApiResponse<Vec<ProductAd>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sp/productAds/list", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request.unwrap_or_default())
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn create_product_ads(
    configuration: &Configuration,
    body: Vec<ProductAd>,
) -> Result<ApiResponse<Vec<ProductAdResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sp/productAds", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn update_product_ads(
    configuration: &Configuration,
    body: Vec<ProductAd>,
) -> Result<ApiResponse<Vec<ProductAdResponse>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::PUT,
            format!("{}/sp/productAds", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn delete_product_ads(
    configuration: &Configuration,
    request: DeleteProductAdsRequest,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/sp/productAds/delete", configuration.base_path),
        )
        .header("Accept", VERSION_HEADER)
        .header("Content-Type", VERSION_HEADER)
        .json(&request)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
