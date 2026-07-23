use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SdBudgetRuleType {
    Schedule,
    Performance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SdBudgetRuleStatus {
    Active,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdBudgetRule {
    pub rule_id: Option<String>,
    pub name: Option<String>,
    pub rule_details: Option<SdBudgetRuleDetails>,
    pub status: Option<SdBudgetRuleStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdBudgetRuleDetails {
    pub duration: Option<SdRuleDuration>,
    pub recurrence: Option<SdRuleRecurrence>,
    pub rule_type: Option<SdBudgetRuleType>,
    pub budget_increase_by: Option<SdBudgetIncreaseBy>,
    pub performance_measure_condition: Option<SdPerformanceMeasureCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdRuleDuration {
    pub event_type_rule_duration: Option<Vec<SdEventTypeRuleDuration>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdEventTypeRuleDuration {
    pub event_id: Option<String>,
    pub end_date: Option<String>,
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdRuleRecurrence {
    #[serde(rename = "type")]
    pub recurrence_type: Option<String>,
    pub days_of_week: Option<Vec<String>>,
    pub intra_day_schedule: Option<Vec<SdIntraDaySchedule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdIntraDaySchedule {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SdBudgetIncreaseBy {
    #[serde(rename = "percent")]
    Percent(f64),
    #[serde(rename = "amount")]
    Amount(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdPerformanceMeasureCondition {
    pub metric_name: Option<String>,
    pub comparison_operator: Option<String>,
    pub threshold: Option<f64>,
}
