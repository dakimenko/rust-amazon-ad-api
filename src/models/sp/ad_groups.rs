use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdGroupState {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AdGroup {
    pub ad_group_id: Option<String>,
    pub name: Option<String>,
    pub campaign_id: Option<String>,
    pub default_bid: Option<f64>,
    pub state: Option<AdGroupState>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AdGroupResponse {
    pub ad_group_id: String,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListAdGroupsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub name_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteAdGroupsRequest {
    pub ad_group_id_filter: super::campaigns::ObjectIdFilter,
}
