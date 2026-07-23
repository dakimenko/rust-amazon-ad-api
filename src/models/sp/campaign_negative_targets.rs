use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CampaignNegativeTargetExpression {
    #[serde(rename = "type")]
    ExpressionType(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct CampaignNegativeTargetingClause {
    pub target_id: Option<String>,
    pub campaign_id: Option<String>,
    pub state: Option<super::targeting::TargetingState>,
    pub expression: Option<Vec<CampaignNegativeTargetExpression>>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListCampaignNegativeTargetsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub target_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCampaignNegativeTargetsRequest {
    pub target_id_filter: super::campaigns::ObjectIdFilter,
}
