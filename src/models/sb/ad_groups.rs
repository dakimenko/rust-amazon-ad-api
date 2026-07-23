use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbAdGroupState {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbAdGroup {
    pub ad_group_id: Option<String>,
    pub name: Option<String>,
    pub campaign_id: Option<String>,
    pub default_bid: Option<f64>,
    pub state: Option<SbAdGroupState>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbAdGroupResponse {
    pub ad_group_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbListAdGroupsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
}
