use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::dsp::reports::*;

pub async fn create_report(
    configuration: &Configuration,
    body: DspReportRequest,
) -> Result<ApiResponse<DspReportResponse>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::POST, format!("{}/v2/dsp/reports", configuration.base_path))
        .header("Accept", "application/vnd.dspcreatereports.v3+json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}

pub async fn get_report_status(
    configuration: &Configuration,
    report_id: &str,
) -> Result<ApiResponse<DspReportResponse>, Error<serde_json::Value>> {
    let req = configuration.client.request(reqwest::Method::GET, format!("{}/v2/dsp/reports/{}", configuration.base_path, report_id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await

}
