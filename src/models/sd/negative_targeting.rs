use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdNegativeTargetingClause {
    pub target_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<super::targeting::SdTargetingState>,
    pub expression: Option<serde_json::Value>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdNegativeTargetingClauseResponse {
    pub target_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdListNegativeTargetsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub target_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdDeleteNegativeTargetsRequest {
    pub target_id_filter: super::campaigns::SdObjectIdFilter,
}
