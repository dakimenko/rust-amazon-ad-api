use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct CampaignOptimizationRule {
    pub rule_id: Option<String>,
    pub campaign_ids: Option<Vec<String>>,
    pub rule_name: Option<String>,
    pub action: Option<String>,
    pub rule_conditions: Option<Vec<OptimizationRuleCondition>>,
    pub rule_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct OptimizationRuleCondition {
    pub metric_name: Option<String>,
    pub comparison_operator: Option<String>,
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct OptimizationEligibilityRequest {
    pub campaign_ids: Option<Vec<String>>,
}
