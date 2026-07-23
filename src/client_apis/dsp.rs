use crate::client::AmazonAdClient;
use crate::models::common::ApiResponse;

impl AmazonAdClient {
    pub async fn dsp_create_report(
        &self,
        body: crate::models::dsp::reports::DspReportRequest,
    ) -> Result<
        ApiResponse<crate::models::dsp::reports::DspReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::dsp::reports::create_report(&config, body).await
    }

    pub async fn dsp_get_report_status(
        &self,
        report_id: &str,
    ) -> Result<
        ApiResponse<crate::models::dsp::reports::DspReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::dsp::reports::get_report_status(&config, report_id).await
    }
}
