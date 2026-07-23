use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbCampaignState {
    Enabled,
    Paused,
    Archived,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbCreativeType {
    Video,
    ProductCollection,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbTargetingType {
    Auto,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbCampaign {
    pub campaign_id: Option<String>,
    pub name: Option<String>,
    pub state: Option<SbCampaignState>,
    pub budget: Option<f64>,
    pub targeting_type: Option<SbTargetingType>,
    pub creative_type: Option<SbCreativeType>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub portfolio_id: Option<i64>,
    pub brand_entity_id: Option<String>,
    pub budget_type: Option<String>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbCampaignResponse {
    pub campaign_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbListCampaignsRequest {
    pub state_filter: Option<String>,
    pub name_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
}
