use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BudgetRuleType {
    Schedule,
    Performance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BudgetRuleStatus {
    Active,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct BudgetRule {
    pub rule_id: Option<String>,
    pub name: Option<String>,
    pub rule_details: Option<BudgetRuleDetails>,
    pub status: Option<BudgetRuleStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct BudgetRuleDetails {
    pub duration: Option<RuleDuration>,
    pub recurrence: Option<RuleRecurrence>,
    pub rule_type: Option<BudgetRuleType>,
    pub budget_increase_by: Option<BudgetIncreaseBy>,
    pub performance_measure_condition: Option<PerformanceMeasureCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct RuleDuration {
    pub event_type_rule_duration: Option<Vec<EventTypeRuleDuration>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct EventTypeRuleDuration {
    pub event_id: Option<String>,
    pub end_date: Option<String>,
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct RuleRecurrence {
    #[serde(rename = "type")]
    pub recurrence_type: Option<String>,
    pub days_of_week: Option<Vec<String>>,
    pub intra_day_schedule: Option<Vec<IntraDaySchedule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct IntraDaySchedule {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BudgetIncreaseBy {
    #[serde(rename = "percent")]
    Percent(f64),
    #[serde(rename = "amount")]
    Amount(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct PerformanceMeasureCondition {
    pub metric_name: Option<String>,
    pub comparison_operator: Option<String>,
    pub threshold: Option<f64>,
}
