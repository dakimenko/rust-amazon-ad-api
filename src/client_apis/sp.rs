// High-level convenience methods for Sponsored Products API.
use crate::client::AmazonAdClient;
use crate::models::common::ApiResponse;

impl AmazonAdClient {
    // ── Campaigns ──────────────────────────────────────────────

    /// List Sponsored Products campaigns (v3).
    pub async fn sp_list_campaigns(
        &self,
        request: Option<crate::models::sp::campaigns::ListCampaignsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::campaigns::Campaign>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaigns::list_campaigns(&config, request).await
    }

    /// Stream Sponsored Products campaigns automatically using cursor pagination.
    pub fn sp_stream_campaigns(
        &self,
        base_request: Option<crate::models::sp::campaigns::ListCampaignsRequest>,
    ) -> impl futures_core::Stream<
        Item = Result<
            crate::models::sp::campaigns::Campaign,
            crate::apis::Error<serde_json::Value>,
        >,
    > {
        let client_clone = self.clone();
        crate::apis::pagination::paginate_cursor(move |cursor| {
            let client = client_clone.clone();
            let mut req = base_request.clone().unwrap_or_default();
            req.next_token = cursor;
            async move { client.sp_list_campaigns(Some(req)).await }
        })
    }

    /// Create Sponsored Products campaigns (v3).
    pub async fn sp_create_campaigns(
        &self,
        body: Vec<crate::models::sp::campaigns::Campaign>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::campaigns::CampaignResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaigns::create_campaigns(&config, body).await
    }

    /// Create a single Sponsored Products campaign (v3).
    pub async fn sp_create_campaign(
        &self,
        campaign: impl Into<crate::models::sp::campaigns::Campaign>,
    ) -> Result<crate::models::sp::campaigns::CampaignResponse, crate::apis::Error<serde_json::Value>>
    {
        let resp = self.sp_create_campaigns(vec![campaign.into()]).await?;
        resp.payload
            .into_iter()
            .next()
            .ok_or_else(|| crate::apis::Error::Custom("Empty response payload".into()))
    }

    /// Update Sponsored Products campaigns (v3).
    pub async fn sp_update_campaigns(
        &self,
        body: Vec<crate::models::sp::campaigns::Campaign>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::campaigns::CampaignResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaigns::update_campaigns(&config, body).await
    }

    /// Delete Sponsored Products campaigns (v3).
    pub async fn sp_delete_campaigns(
        &self,
        request: crate::models::sp::campaigns::DeleteCampaignsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaigns::delete_campaigns(&config, request).await
    }

    // ── Ad Groups ──────────────────────────────────────────────

    /// List Sponsored Products ad groups (v3).
    pub async fn sp_list_ad_groups(
        &self,
        request: Option<crate::models::sp::ad_groups::ListAdGroupsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::ad_groups::AdGroup>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::ad_groups::list_ad_groups(&config, request).await
    }

    /// Create Sponsored Products ad groups (v3).
    pub async fn sp_create_ad_groups(
        &self,
        body: Vec<crate::models::sp::ad_groups::AdGroup>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::ad_groups::AdGroupResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::ad_groups::create_ad_groups(&config, body).await
    }

    /// Update Sponsored Products ad groups (v3).
    pub async fn sp_update_ad_groups(
        &self,
        body: Vec<crate::models::sp::ad_groups::AdGroup>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::ad_groups::AdGroupResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::ad_groups::update_ad_groups(&config, body).await
    }

    /// Delete Sponsored Products ad groups (v3).
    pub async fn sp_delete_ad_groups(
        &self,
        request: crate::models::sp::ad_groups::DeleteAdGroupsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::ad_groups::delete_ad_groups(&config, request).await
    }

    // ── Keywords ───────────────────────────────────────────────

    /// List Sponsored Products keywords (v3).
    pub async fn sp_list_keywords(
        &self,
        request: Option<crate::models::sp::keywords::ListKeywordsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::keywords::Keyword>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::keywords::list_keywords(&config, request).await
    }

    /// Create Sponsored Products keywords (v3).
    pub async fn sp_create_keywords(
        &self,
        body: Vec<crate::models::sp::keywords::Keyword>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::keywords::KeywordResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::keywords::create_keywords(&config, body).await
    }

    /// Update Sponsored Products keywords (v3).
    pub async fn sp_update_keywords(
        &self,
        body: Vec<crate::models::sp::keywords::Keyword>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::keywords::KeywordResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::keywords::update_keywords(&config, body).await
    }

    /// Delete Sponsored Products keywords (v3).
    pub async fn sp_delete_keywords(
        &self,
        request: crate::models::sp::keywords::DeleteKeywordsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::keywords::delete_keywords(&config, request).await
    }

    // ── Negative Keywords ──────────────────────────────────────

    /// List Sponsored Products negative keywords (v3).
    pub async fn sp_list_negative_keywords(
        &self,
        request: Option<crate::models::sp::negative_keywords::ListNegativeKeywordsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::negative_keywords::NegativeKeyword>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_keywords::list_negative_keywords(&config, request).await
    }

    /// Create Sponsored Products negative keywords (v3).
    pub async fn sp_create_negative_keywords(
        &self,
        body: Vec<crate::models::sp::negative_keywords::NegativeKeyword>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::negative_keywords::NegativeKeywordResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_keywords::create_negative_keywords(&config, body).await
    }

    /// Update Sponsored Products negative keywords (v3).
    pub async fn sp_update_negative_keywords(
        &self,
        body: Vec<crate::models::sp::negative_keywords::NegativeKeyword>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::negative_keywords::NegativeKeywordResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_keywords::update_negative_keywords(&config, body).await
    }

    /// Delete Sponsored Products negative keywords (v3).
    pub async fn sp_delete_negative_keywords(
        &self,
        request: crate::models::sp::negative_keywords::DeleteNegativeKeywordsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_keywords::delete_negative_keywords(&config, request).await
    }

    // ── Product Ads ────────────────────────────────────────────

    /// List Sponsored Products product ads (v3).
    pub async fn sp_list_product_ads(
        &self,
        request: Option<crate::models::sp::product_ads::ListProductAdsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::product_ads::ProductAd>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_ads::list_product_ads(&config, request).await
    }

    /// Create Sponsored Products product ads (v3).
    pub async fn sp_create_product_ads(
        &self,
        body: Vec<crate::models::sp::product_ads::ProductAd>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::product_ads::ProductAdResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_ads::create_product_ads(&config, body).await
    }

    /// Update Sponsored Products product ads (v3).
    pub async fn sp_update_product_ads(
        &self,
        body: Vec<crate::models::sp::product_ads::ProductAd>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::product_ads::ProductAdResponse>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_ads::update_product_ads(&config, body).await
    }

    /// Delete Sponsored Products product ads (v3).
    pub async fn sp_delete_product_ads(
        &self,
        request: crate::models::sp::product_ads::DeleteProductAdsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_ads::delete_product_ads(&config, request).await
    }

    // ── Product Targeting ──────────────────────────────────────

    /// List Sponsored Products targeting clauses (v3).
    pub async fn sp_list_targets(
        &self,
        request: Option<crate::models::sp::targeting::ListTargetsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::targeting::TargetingClause>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_targeting::list_targets(&config, request).await
    }

    /// Create Sponsored Products targeting clauses (v3).
    pub async fn sp_create_targets(
        &self,
        body: Vec<crate::models::sp::targeting::TargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_targeting::create_targets(&config, body).await
    }

    /// Update Sponsored Products targeting clauses (v3).
    pub async fn sp_update_targets(
        &self,
        body: Vec<crate::models::sp::targeting::TargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_targeting::update_targets(&config, body).await
    }

    /// Delete Sponsored Products targeting clauses (v3).
    pub async fn sp_delete_targets(
        &self,
        request: crate::models::sp::targeting::DeleteTargetsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::product_targeting::delete_targets(&config, request).await
    }

    // ── Negative Product Targeting ─────────────────────────────

    /// List Sponsored Products negative targeting clauses (v3).
    pub async fn sp_list_negative_targets(
        &self,
        request: Option<crate::models::sp::negative_targeting::ListNegativeTargetsRequest>,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::negative_targeting::NegativeTargetingClause>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_product_targeting::list_negative_targets(&config, request).await
    }

    /// Create Sponsored Products negative targeting clauses (v3).
    pub async fn sp_create_negative_targets(
        &self,
        body: Vec<crate::models::sp::negative_targeting::NegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_product_targeting::create_negative_targets(&config, body).await
    }

    /// Update Sponsored Products negative targeting clauses (v3).
    pub async fn sp_update_negative_targets(
        &self,
        body: Vec<crate::models::sp::negative_targeting::NegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_product_targeting::update_negative_targets(&config, body).await
    }

    /// Delete Sponsored Products negative targeting clauses (v3).
    pub async fn sp_delete_negative_targets(
        &self,
        request: crate::models::sp::negative_targeting::DeleteNegativeTargetsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::negative_product_targeting::delete_negative_targets(&config, request).await
    }

    // ── Campaign Negative Keywords ─────────────────────────────

    /// List Sponsored Products campaign negative keywords (v3).
    pub async fn sp_list_campaign_negative_keywords(
        &self,
        request: Option<
            crate::models::sp::campaign_negative_keywords::ListCampaignNegativeKeywordsRequest,
        >,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::campaign_negative_keywords::CampaignNegativeKeyword>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_keywords::list_campaign_negative_keywords(
            &config, request,
        )
        .await
    }

    /// Create Sponsored Products campaign negative keywords (v3).
    pub async fn sp_create_campaign_negative_keywords(
        &self,
        body: Vec<crate::models::sp::campaign_negative_keywords::CampaignNegativeKeyword>,
    ) -> Result<
        ApiResponse<
            Vec<crate::models::sp::campaign_negative_keywords::CampaignNegativeKeywordResponse>,
        >,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_keywords::create_campaign_negative_keywords(
            &config, body,
        )
        .await
    }

    /// Update Sponsored Products campaign negative keywords (v3).
    pub async fn sp_update_campaign_negative_keywords(
        &self,
        body: Vec<crate::models::sp::campaign_negative_keywords::CampaignNegativeKeyword>,
    ) -> Result<
        ApiResponse<
            Vec<crate::models::sp::campaign_negative_keywords::CampaignNegativeKeywordResponse>,
        >,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_keywords::update_campaign_negative_keywords(
            &config, body,
        )
        .await
    }

    /// Delete Sponsored Products campaign negative keywords (v3).
    pub async fn sp_delete_campaign_negative_keywords(
        &self,
        request: crate::models::sp::campaign_negative_keywords::DeleteCampaignNegativeKeywordsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_keywords::delete_campaign_negative_keywords(
            &config, request,
        )
        .await
    }

    // ── Campaign Negative Targets ──────────────────────────────

    /// List Sponsored Products campaign negative targets (v3).
    pub async fn sp_list_campaign_negative_targets(
        &self,
        request: Option<
            crate::models::sp::campaign_negative_targets::ListCampaignNegativeTargetsRequest,
        >,
    ) -> Result<
        ApiResponse<
            Vec<crate::models::sp::campaign_negative_targets::CampaignNegativeTargetingClause>,
        >,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_targets::list_campaign_negative_targets(&config, request)
            .await
    }

    /// Create Sponsored Products campaign negative targets (v3).
    pub async fn sp_create_campaign_negative_targets(
        &self,
        body: Vec<crate::models::sp::campaign_negative_targets::CampaignNegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_targets::create_campaign_negative_targets(&config, body)
            .await
    }

    /// Update Sponsored Products campaign negative targets (v3).
    pub async fn sp_update_campaign_negative_targets(
        &self,
        body: Vec<crate::models::sp::campaign_negative_targets::CampaignNegativeTargetingClause>,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_targets::update_campaign_negative_targets(&config, body)
            .await
    }

    /// Delete Sponsored Products campaign negative targets (v3).
    pub async fn sp_delete_campaign_negative_targets(
        &self,
        request: crate::models::sp::campaign_negative_targets::DeleteCampaignNegativeTargetsRequest,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::campaign_negative_targets::delete_campaign_negative_targets(
            &config, request,
        )
        .await
    }

    // ── Reports ────────────────────────────────────────────────

    /// Create a Sponsored Products report.
    pub async fn sp_create_report(
        &self,
        record_type: &str,
        body: crate::models::sp::reports::SpReportRequest,
    ) -> Result<
        ApiResponse<crate::models::sp::reports::ReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::reports::create_report(&config, record_type, body).await
    }

    /// Get Sponsored Products report status.
    pub async fn sp_get_report_status(
        &self,
        report_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sp::reports::ReportResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::reports::get_report_status(&config, report_id).await
    }

    // ── Snapshots ──────────────────────────────────────────────

    /// Create a Sponsored Products snapshot.
    pub async fn sp_create_snapshot(
        &self,
        body: crate::models::sp::snapshots::SnapshotRequest,
    ) -> Result<
        ApiResponse<crate::models::sp::snapshots::SnapshotResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::snapshots::create_snapshot(&config, body).await
    }

    /// Get Sponsored Products snapshot status.
    pub async fn sp_get_snapshot_status(
        &self,
        snapshot_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sp::snapshots::SnapshotStatusResponse>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::snapshots::get_snapshot_status(&config, snapshot_id).await
    }

    // ── Budget Rules ───────────────────────────────────────────

    /// List Sponsored Products budget rules.
    pub async fn sp_list_budget_rules(
        &self,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::budget_rules::BudgetRule>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::budget_rules::list_budget_rules(&config).await
    }

    /// Create a Sponsored Products budget rule.
    pub async fn sp_create_budget_rule(
        &self,
        body: Vec<crate::models::sp::budget_rules::BudgetRule>,
    ) -> Result<ApiResponse<Vec<serde_json::Value>>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::budget_rules::create_budget_rule(&config, body).await
    }

    /// Get a Sponsored Products budget rule.
    pub async fn sp_get_budget_rule(
        &self,
        rule_id: &str,
    ) -> Result<
        ApiResponse<crate::models::sp::budget_rules::BudgetRule>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::budget_rules::get_budget_rule(&config, rule_id).await
    }

    /// Update a Sponsored Products budget rule.
    pub async fn sp_update_budget_rule(
        &self,
        body: Vec<crate::models::sp::budget_rules::BudgetRule>,
    ) -> Result<ApiResponse<Vec<serde_json::Value>>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::budget_rules::update_budget_rule(&config, body).await
    }

    /// Delete a Sponsored Products budget rule.
    pub async fn sp_delete_budget_rule(
        &self,
        rule_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>> {
        let config = self.create_configuration().await?;
        crate::apis::sp::budget_rules::delete_budget_rule(&config, rule_id).await
    }

    // ── Bid Recommendations ────────────────────────────────────

    /// Get Sponsored Products bid recommendations.
    pub async fn sp_get_bid_recommendations(
        &self,
        body: crate::models::sp::bids::BidRecommendationsRequest,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::bids::BidRecommendation>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::bid_recommendations::get_bid_recommendations(&config, body).await
    }

    // ── Keyword Suggestions ────────────────────────────────────

    /// Get Sponsored Products suggested keywords.
    pub async fn sp_get_suggested_keywords(
        &self,
        body: crate::models::sp::recommendations::SuggestedKeywordsRequest,
    ) -> Result<
        ApiResponse<Vec<crate::models::sp::recommendations::SuggestedKeyword>>,
        crate::apis::Error<serde_json::Value>,
    > {
        let config = self.create_configuration().await?;
        crate::apis::sp::suggested_keywords::get_suggested_keywords(&config, body).await
    }
}
