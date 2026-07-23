use crate::client::AmazonAdClient;
use crate::models::common::ApiResponse;

impl AmazonAdClient {
    // ── Campaigns ──────────────────────────────────────────────

    pub async fn sb_list_campaigns(
        &self,
        request: Option<crate::models::sb::campaigns::SbListCampaignsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::campaigns::SbCampaign>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::campaigns::list_campaigns(&config, request).await
    }

    pub async fn sb_create_campaigns(
        &self,
        body: Vec<crate::models::sb::campaigns::SbCampaign>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::campaigns::SbCampaignResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::campaigns::create_campaigns(&config, body).await
    }

    pub async fn sb_update_campaigns(
        &self,
        body: Vec<crate::models::sb::campaigns::SbCampaign>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::campaigns::SbCampaignResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::campaigns::update_campaigns(&config, body).await
    }

    // ── Ad Groups ──────────────────────────────────────────────

    pub async fn sb_list_ad_groups(
        &self,
        request: Option<crate::models::sb::ad_groups::SbListAdGroupsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ad_groups::SbAdGroup>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ad_groups::list_ad_groups(&config, request).await
    }

    pub async fn sb_create_ad_groups(
        &self,
        body: Vec<crate::models::sb::ad_groups::SbAdGroup>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ad_groups::SbAdGroupResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ad_groups::create_ad_groups(&config, body).await
    }

    pub async fn sb_update_ad_groups(
        &self,
        body: Vec<crate::models::sb::ad_groups::SbAdGroup>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ad_groups::SbAdGroupResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ad_groups::update_ad_groups(&config, body).await
    }

    // ── Creatives ──────────────────────────────────────────────

    pub async fn sb_list_creatives(
        &self,
        request: Option<crate::models::sb::creatives::SbListCreativesRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::creatives::SbCreative>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::creatives::list_creatives(&config, request).await
    }

    pub async fn sb_create_creatives(
        &self,
        body: Vec<crate::models::sb::creatives::SbCreative>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::creatives::SbCreativeResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::creatives::create_creatives(&config, body).await
    }

    pub async fn sb_update_creatives(
        &self,
        body: Vec<crate::models::sb::creatives::SbCreative>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::creatives::SbCreativeResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::creatives::update_creatives(&config, body).await
    }

    // ── Ads ────────────────────────────────────────────────────

    pub async fn sb_list_ads(
        &self,
        request: Option<crate::models::sb::ads::SbListAdsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ads::SbAd>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ads::list_ads(&config, request).await
    }

    pub async fn sb_create_ads(
        &self,
        body: Vec<crate::models::sb::ads::SbAd>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ads::SbAdResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ads::create_ads(&config, body).await
    }

    pub async fn sb_update_ads(
        &self,
        body: Vec<crate::models::sb::ads::SbAd>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::ads::SbAdResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::ads::update_ads(&config, body).await
    }

    // ── Keywords ───────────────────────────────────────────────

    pub async fn sb_list_keywords(
        &self,
        request: Option<crate::models::sb::keywords::SbListKeywordsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::keywords::SbKeyword>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::keywords::list_keywords(&config, request).await
    }

    pub async fn sb_create_keywords(
        &self,
        body: Vec<crate::models::sb::keywords::SbKeyword>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::keywords::SbKeywordResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::keywords::create_keywords(&config, body).await
    }

    pub async fn sb_update_keywords(
        &self,
        body: Vec<crate::models::sb::keywords::SbKeyword>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::keywords::SbKeywordResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::keywords::update_keywords(&config, body).await
    }

    pub async fn sb_get_keyword(
        &self,
        keyword_id: &str,
    ) -> Result<ApiResponse<crate::models::sb::keywords::SbKeyword>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::keywords::get_keyword(&config, keyword_id).await
    }

    pub async fn sb_delete_keyword(
        &self,
        keyword_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::keywords::delete_keyword(&config, keyword_id).await
    }

    // ── Negative Keywords ──────────────────────────────────────

    pub async fn sb_list_negative_keywords(
        &self,
        request: Option<crate::models::sb::negative_keywords::SbListNegativeKeywordsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_keywords::SbNegativeKeyword>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_keywords::list_negative_keywords(&config, request).await
    }

    pub async fn sb_create_negative_keywords(
        &self,
        body: Vec<crate::models::sb::negative_keywords::SbNegativeKeyword>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_keywords::SbNegativeKeywordResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_keywords::create_negative_keywords(&config, body).await
    }

    pub async fn sb_update_negative_keywords(
        &self,
        body: Vec<crate::models::sb::negative_keywords::SbNegativeKeyword>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_keywords::SbNegativeKeywordResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_keywords::update_negative_keywords(&config, body).await
    }

    pub async fn sb_delete_negative_keyword(
        &self,
        keyword_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_keywords::delete_negative_keyword(&config, keyword_id).await
    }

    // ── Product Targeting ──────────────────────────────────────

    pub async fn sb_list_targets(
        &self,
        request: Option<crate::models::sb::targeting::SbListTargetsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::targeting::SbTargetingClause>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::product_targeting::list_targets(&config, request).await
    }

    pub async fn sb_create_targets(
        &self,
        body: Vec<crate::models::sb::targeting::SbTargetingClause>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::targeting::SbTargetingClauseResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::product_targeting::create_targets(&config, body).await
    }

    pub async fn sb_update_targets(
        &self,
        body: Vec<crate::models::sb::targeting::SbTargetingClause>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::targeting::SbTargetingClauseResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::product_targeting::update_targets(&config, body).await
    }

    pub async fn sb_delete_target(
        &self,
        target_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::product_targeting::delete_target(&config, target_id).await
    }

    // ── Negative Product Targeting ─────────────────────────────

    pub async fn sb_list_negative_targets(
        &self,
        request: Option<crate::models::sb::negative_targeting::SbListNegativeTargetsRequest>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_targeting::SbNegativeTargetingClause>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_product_targeting::list_negative_targets(&config, request).await
    }

    pub async fn sb_create_negative_targets(
        &self,
        body: Vec<crate::models::sb::negative_targeting::SbNegativeTargetingClause>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_targeting::SbNegativeTargetingClauseResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_product_targeting::create_negative_targets(&config, body).await
    }

    pub async fn sb_update_negative_targets(
        &self,
        body: Vec<crate::models::sb::negative_targeting::SbNegativeTargetingClause>,
    ) -> Result<ApiResponse<Vec<crate::models::sb::negative_targeting::SbNegativeTargetingClauseResponse>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_product_targeting::update_negative_targets(&config, body).await
    }

    pub async fn sb_delete_negative_target(
        &self,
        target_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::negative_product_targeting::delete_negative_target(&config, target_id).await
    }

    // ── Media ──────────────────────────────────────────────────

    pub async fn sb_upload_media(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<ApiResponse<crate::models::sb::media::SbMediaUploadResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::media::upload_media(&config, form).await
    }

    pub async fn sb_get_media_status(
        &self,
        media_id: &str,
    ) -> Result<ApiResponse<crate::models::sb::media::SbMediaUploadResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::media::get_media_status(&config, media_id).await
    }

    pub async fn sb_complete_media(
        &self,
        media_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::media::complete_media(&config, media_id).await
    }

    // ── Benchmarks ─────────────────────────────────────────────

    pub async fn sb_get_benchmarks(
        &self,
        keyword_or_target: &str,
        body: crate::models::sb::benchmarks::SbBenchmarkRequest,
    ) -> Result<ApiResponse<Vec<crate::models::sb::benchmarks::SbBenchmarkResult>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::benchmarks::get_benchmarks(&config, keyword_or_target, body).await
    }

    // ── Themes ─────────────────────────────────────────────────

    pub async fn sb_get_themes(
        &self,
    ) -> Result<ApiResponse<Vec<crate::models::sb::themes::SbTheme>>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::themes::get_themes(&config).await
    }

    // ── Reports ────────────────────────────────────────────────

    pub async fn sb_create_report(
        &self,
        record_type: &str,
        body: crate::models::sb::reports::SbReportRequest,
    ) -> Result<ApiResponse<crate::models::sb::reports::SbReportResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::reports::create_report(&config, record_type, body).await
    }

    pub async fn sb_get_report_status(
        &self,
        report_id: &str,
    ) -> Result<ApiResponse<crate::models::sb::reports::SbReportResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::reports::get_report_status(&config, report_id).await
    }

    // ── Snapshots ──────────────────────────────────────────────

    pub async fn sb_create_snapshot(
        &self,
        body: crate::models::sb::snapshots::SbSnapshotRequest,
    ) -> Result<ApiResponse<crate::models::sb::snapshots::SbSnapshotResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::snapshots::create_snapshot(&config, body).await
    }

    pub async fn sb_get_snapshot_status(
        &self,
        snapshot_id: &str,
    ) -> Result<ApiResponse<crate::models::sb::snapshots::SbSnapshotStatusResponse>, crate::apis::Error<serde_json::Value>>
    {
        let config = self.create_configuration().await?;
        crate::apis::sb::snapshots::get_snapshot_status(&config, snapshot_id).await
    }
}
