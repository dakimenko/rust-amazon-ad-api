use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::reports::*;

pub async fn create_report(
    configuration: &Configuration,
    record_type: &str,
    body: SbReportRequest,
) -> Result<ApiResponse<SbReportResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!("{}/v2/hsa/{}/report", configuration.base_path, record_type),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}

pub async fn get_report_status(
    configuration: &Configuration,
    report_id: &str,
) -> Result<ApiResponse<SbReportResponse>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::GET,
            format!("{}/v2/reports/{}", configuration.base_path, report_id),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
