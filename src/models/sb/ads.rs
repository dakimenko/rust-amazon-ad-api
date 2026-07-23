use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbAdState { Enabled, Paused, Archived }

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbAd {
    pub ad_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub creative_id: Option<String>,
    pub state: Option<SbAdState>,
    pub asin: Option<String>,
    pub sku: Option<String>,
    pub landing_page_url: Option<String>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbAdResponse {
    pub ad_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbListAdsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub creative_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
}
