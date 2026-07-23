use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::sb::themes::*;
use crate::models::common::ApiResponse;

pub async fn get_themes(
    configuration: &Configuration,
) -> Result<ApiResponse<Vec<SbTheme>>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/v2/hsa/themes", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
