use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::cross::reports::*;

pub async fn create_report(
    configuration: &Configuration,
    body: ReportRequestV3,
) -> Result<ApiResponse<ReportResponseV3>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/v3/reports", configuration.base_path))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_report_status(
    configuration: &Configuration,
    report_id: &str,
) -> Result<ApiResponse<ReportResponseV3>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/v3/reports/{}", configuration.base_path, report_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn delete_report(
    configuration: &Configuration,
    report_id: &str,
) -> Result<ApiResponse<serde_json::Value>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::DELETE, format!("{}/v3/reports/{}", configuration.base_path, report_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    let resp = configuration.client.execute(req).await?;
    let status = resp.status();
    let content = resp.text().await?;
    if status.is_success() {
        let payload: serde_json::Value = if content.is_empty() { serde_json::Value::Null } else { serde_json::from_str(&content)? };
        Ok(ApiResponse { payload, headers: Default::default(), next_token: None, rate_limit: None })
    } else {
        Err(Error::ResponseError(crate::apis::ResponseContent { status, content, entity: None }))
    }
}
