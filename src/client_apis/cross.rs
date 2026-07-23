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
}
