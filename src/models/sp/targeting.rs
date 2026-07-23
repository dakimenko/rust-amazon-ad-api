use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetingState {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetingExpression {
    #[serde(rename = "type")]
    ExpressionType(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct TargetingClause {
    pub target_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<TargetingState>,
    pub expression: Option<Vec<TargetingExpression>>,
    pub expression_type: Option<String>,
    pub bid: Option<f64>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListTargetsRequest {
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
pub struct DeleteTargetsRequest {
    pub target_id_filter: super::campaigns::ObjectIdFilter,
}
