use crate::client::AmazonAdClient;
use crate::models::common::ApiResponse;

impl AmazonAdClient {
    // ── Campaigns ──────────────────────────────────────────────

    pub async fn sd_list_campaigns(
        &self,
        request: Option<crate::models::sd::campaigns::SdListCampaignsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::campaigns::SdCampaign>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::campaigns::list_campaigns(&config, request).await
    }

    pub async fn sd_create_campaigns(
        &self,
        body: Vec<crate::models::sd::campaigns::SdCampaign>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::campaigns::SdCampaignResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::campaigns::create_campaigns(&config, body).await
    }

    pub async fn sd_update_campaigns(
        &self,
        body: Vec<crate::models::sd::campaigns::SdCampaign>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::campaigns::SdCampaignResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::campaigns::update_campaigns(&config, body).await
    }

    pub async fn sd_delete_campaigns(
        &self,
        request: crate::models::sd::campaigns::SdDeleteCampaignsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::campaigns::delete_campaigns(&config, request).await
    }

    // ── Ad Groups ──────────────────────────────────────────────

    pub async fn sd_list_ad_groups(
        &self,
        request: Option<crate::models::sd::ad_groups::SdListAdGroupsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::ad_groups::SdAdGroup>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::ad_groups::list_ad_groups(&config, request).await
    }

    pub async fn sd_create_ad_groups(
        &self,
        body: Vec<crate::models::sd::ad_groups::SdAdGroup>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::ad_groups::SdAdGroupResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::ad_groups::create_ad_groups(&config, body).await
    }

    pub async fn sd_update_ad_groups(
        &self,
        body: Vec<crate::models::sd::ad_groups::SdAdGroup>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::ad_groups::SdAdGroupResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::ad_groups::update_ad_groups(&config, body).await
    }

    pub async fn sd_delete_ad_groups(
        &self,
        request: crate::models::sd::ad_groups::SdDeleteAdGroupsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::ad_groups::delete_ad_groups(&config, request).await
    }

    // ── Product Ads ────────────────────────────────────────────

    pub async fn sd_list_product_ads(
        &self,
        request: Option<crate::models::sd::product_ads::SdListProductAdsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::product_ads::SdProductAd>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_ads::list_product_ads(&config, request).await
    }

    pub async fn sd_create_product_ads(
        &self,
        body: Vec<crate::models::sd::product_ads::SdProductAd>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::product_ads::SdProductAdResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_ads::create_product_ads(&config, body).await
    }

    pub async fn sd_update_product_ads(
        &self,
        body: Vec<crate::models::sd::product_ads::SdProductAd>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::product_ads::SdProductAdResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_ads::update_product_ads(&config, body).await
    }

    pub async fn sd_delete_product_ads(
        &self,
        request: crate::models::sd::product_ads::SdDeleteProductAdsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_ads::delete_product_ads(&config, request).await
    }

    // ── Product Targeting ──────────────────────────────────────

    pub async fn sd_list_targets(
        &self,
        request: Option<crate::models::sd::targeting::SdListTargetsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::targeting::SdTargetingClause>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_targeting::list_targets(&config, request).await
    }

    pub async fn sd_create_targets(
        &self,
        body: Vec<crate::models::sd::targeting::SdTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_targeting::create_targets(&config, body).await
    }

    pub async fn sd_update_targets(
        &self,
        body: Vec<crate::models::sd::targeting::SdTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_targeting::update_targets(&config, body).await
    }

    pub async fn sd_delete_targets(
        &self,
        request: crate::models::sd::targeting::SdDeleteTargetsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::product_targeting::delete_targets(&config, request).await
    }

    // ── Negative Product Targeting ─────────────────────────────

    pub async fn sd_list_negative_targets(
        &self,
        request: Option<crate::models::sd::negative_targeting::SdListNegativeTargetsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::negative_targeting::SdNegativeTargetingClause>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::negative_product_targeting::list_negative_targets(&config, request).await
    }

    pub async fn sd_create_negative_targets(
        &self,
        body: Vec<crate::models::sd::negative_targeting::SdNegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::negative_product_targeting::create_negative_targets(&config, body).await
    }

    pub async fn sd_update_negative_targets(
        &self,
        body: Vec<crate::models::sd::negative_targeting::SdNegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::negative_product_targeting::update_negative_targets(&config, body).await
    }

    pub async fn sd_delete_negative_targets(
        &self,
        request: crate::models::sd::negative_targeting::SdDeleteNegativeTargetsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::negative_product_targeting::delete_negative_targets(&config, request).await
    }

    // ── Creatives ──────────────────────────────────────────────

    pub async fn sd_list_creatives(
        &self,
        request: Option<crate::models::sd::creatives::SdListCreativesRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::creatives::SdCreative>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::creatives::list_creatives(&config, request).await
    }

    pub async fn sd_create_creatives(
        &self,
        body: Vec<crate::models::sd::creatives::SdCreative>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::creatives::SdCreativeResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::creatives::create_creatives(&config, body).await
    }

    pub async fn sd_update_creatives(
        &self,
        body: Vec<crate::models::sd::creatives::SdCreative>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::creatives::SdCreativeResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::creatives::update_creatives(&config, body).await
    }

    // ── Brand Safety ───────────────────────────────────────────

    pub async fn sd_get_deny_list(
        &self,
    ) -> Result<
        ApiResponse<crate::models::sd::brand_safety::SdBrandSafetyDenyList>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::brand_safety::get_deny_list(&config).await
    }

    pub async fn sd_update_deny_list(
        &self,
        body: crate::models::sd::brand_safety::SdBrandSafetyDenyList,
    ) -> Result<
        ApiResponse<crate::models::sd::brand_safety::SdBrandSafetyDenyList>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::brand_safety::update_deny_list(&config, body).await
    }

    // ── Budget Rules ───────────────────────────────────────────

    pub async fn sd_list_budget_rules(
        &self,
    ) -> Result<
        ApiResponse<Vec<crate::models::sd::budget_rules::SdBudgetRule>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::budget_rules::list_budget_rules(&config).await
    }

    pub async fn sd_get_budget_rule(
        &self,
        rule_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sd::budget_rules::SdBudgetRule>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::budget_rules::get_budget_rule(&config, rule_id).await
    }

    pub async fn sd_create_budget_rule(
        &self,
        body: Vec<crate::models::sd::budget_rules::SdBudgetRule>,
    ) -> Result<ApiResponse<Vec<serde_json::Value>>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::budget_rules::create_budget_rule(&config, body).await
    }

    pub async fn sd_update_budget_rule(
        &self,
        body: Vec<crate::models::sd::budget_rules::SdBudgetRule>,
    ) -> Result<ApiResponse<Vec<serde_json::Value>>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::budget_rules::update_budget_rule(&config, body).await
    }

    pub async fn sd_delete_budget_rule(
        &self,
        rule_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sd::budget_rules::delete_budget_rule(&config, rule_id).await
    }

    // ── Reports ────────────────────────────────────────────────

    pub async fn sd_create_report(
        &self,
        record_type: &str,
        body: crate::models::sd::reports::SdReportRequest,
    ) -> Result<
        ApiResponse<crate::models::sd::reports::SdReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::reports::create_report(&config, record_type, body).await
    }

    pub async fn sd_get_report_status(
        &self,
        report_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sd::reports::SdReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::reports::get_report_status(&config, report_id).await
    }

    // ── Snapshots ──────────────────────────────────────────────

    pub async fn sd_create_snapshot(
        &self,
        body: crate::models::sd::snapshots::SdSnapshotRequest,
    ) -> Result<
        ApiResponse<crate::models::sd::snapshots::SdSnapshotResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::snapshots::create_snapshot(&config, body).await
    }

    pub async fn sd_get_snapshot_status(
        &self,
        snapshot_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sd::snapshots::SdSnapshotStatusResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sd::snapshots::get_snapshot_status(&config, snapshot_id).await
    }
}
