use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;

pub async fn get_recommendations(
    configuration: &Configuration,
    query: Option<&[(String, String)]>,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut req = client
        .request(reqwest::Method::GET, format!("{}/v2/hsa/recommendations", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
    if let Some(params) = query {
        req = req.query(&params);
    }
    let req = req.build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
