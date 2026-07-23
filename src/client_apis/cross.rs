use crate::client::AmazonAdClient;
use crate::models::common::ApiResponse;

impl AmazonAdClient {
    pub async fn cross_list_profiles(
        &self,
    ) -> Result<
        ApiResponse<Vec<crate::models::cross::profiles::Profile>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::profiles::list_profiles(&config).await
    }

    pub async fn cross_get_profile(
        &self,
        profile_id: &str,
    ) -> Result<
        ApiResponse<crate::models::cross::profiles::Profile>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::profiles::get_profile(&config, profile_id).await
    }

    pub async fn cross_register_profile(
        &self,
        body: crate::models::cross::profiles::RegisterProfileRequest,
    ) -> Result<
        ApiResponse<crate::models::cross::profiles::Profile>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::profiles::register_profile(&config, body).await
    }

    pub async fn cross_list_portfolios(
        &self,
    ) -> Result<
        ApiResponse<Vec<crate::models::cross::portfolios::Portfolio>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::portfolios::list_portfolios(&config).await
    }

    pub async fn cross_create_portfolios(
        &self,
        body: Vec<crate::models::cross::portfolios::Portfolio>,
    ) -> Result<
        ApiResponse<Vec<crate::models::cross::portfolios::Portfolio>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::portfolios::create_portfolios(&config, body).await
    }

    pub async fn cross_update_portfolios(
        &self,
        body: Vec<crate::models::cross::portfolios::Portfolio>,
    ) -> Result<
        ApiResponse<Vec<crate::models::cross::portfolios::Portfolio>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::portfolios::update_portfolios(&config, body).await
    }

    pub async fn cross_delete_portfolio(
        &self,
        portfolio_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::cross::portfolios::delete_portfolio(&config, portfolio_id).await
    }

    pub async fn cross_create_report(
        &self,
        body: crate::models::cross::reports::ReportRequestV3,
    ) -> Result<
        ApiResponse<crate::models::cross::reports::ReportResponseV3>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::reports::create_report(&config, body).await
    }

    pub async fn cross_get_report_status(
        &self,
        report_id: &str,
    ) -> Result<
        ApiResponse<crate::models::cross::reports::ReportResponseV3>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::cross::reports::get_report_status(&config, report_id).await
    }

    pub async fn cross_delete_report(
        &self,
        report_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::cross::reports::delete_report(&config, report_id).await
    }

    /// Poll a report until it reaches `COMPLETED` status, then download and decompress it.
    ///
    /// This helper automates the full Unified Reports v3 lifecycle:
    /// 1. Polls `GET /v3/reports/{reportId}` with exponential backoff.
    /// 2. On `COMPLETED`, downloads the presigned S3 URL via streaming gzip decompression.
    /// 3. Returns the raw decompressed bytes.
    ///
    /// # Arguments
    /// * `report_id` — The report ID returned by `cross_create_report`.
    /// * `max_polls` — Maximum number of polling attempts before returning an error.
    /// * `initial_delay_ms` — Initial delay in milliseconds between polls (doubles each attempt, capped at 30s).
    pub async fn cross_download_report_polled(
        &self,
        report_id: &str,
        max_polls: u32,
        initial_delay_ms: u64,
    ) -> Result<Vec<u8>, crate::apis::Error<serde_json::Value>> {
        use std::time::Duration;

        let mut delay_ms = initial_delay_ms;
        let mut attempts = 0u32;

        loop {
            if attempts >= max_polls {
                return Err(crate::apis::Error::Custom(format!(
                    "Report {} did not complete after {} polling attempts",
                    report_id, max_polls
                )));
            }

            let status_resp = self.cross_get_report_status(report_id).await?;
            let report = &status_resp.payload;

            match report.status.as_deref() {
                Some("COMPLETED") => {
                    let url = report
                        .url
                        .as_deref()
                        .or(report.location.as_deref())
                        .ok_or_else(|| {
                            crate::apis::Error::Custom(
                                "Report COMPLETED but no download URL provided".into(),
                            )
                        })?;

                    tracing::info!(
                        "Report {} completed. Downloading from presigned URL.",
                        report_id
                    );

                    let bytes = crate::client::download::download(
                        self.get_http_client(),
                        url,
                        crate::client::download::DownloadFormat::Raw,
                    )
                    .await
                    .map_err(|e| crate::apis::Error::Custom(e.to_string()))?;

                    let raw = match bytes {
                        crate::client::download::DownloadResult::Raw(b) => b.to_vec(),
                        _ => {
                            return Err(crate::apis::Error::Custom(
                                "Unexpected download result variant".into(),
                            ))
                        }
                    };

                    return Ok(raw);
                }
                Some("FATAL" | "FAILURE") => {
                    return Err(crate::apis::Error::Custom(format!(
                        "Report {} failed with status: {} — {}",
                        report_id,
                        report.status.as_deref().unwrap_or("UNKNOWN"),
                        report.status_details.as_deref().unwrap_or("")
                    )));
                }
                status => {
                    tracing::debug!(
                        "Report {} status: {:?}. Poll {}/{}. Waiting {}ms...",
                        report_id,
                        status,
                        attempts + 1,
                        max_polls,
                        delay_ms
                    );
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    delay_ms = (delay_ms * 2).min(30_000);
                    attempts += 1;
                }
            }
        }
    }
}
