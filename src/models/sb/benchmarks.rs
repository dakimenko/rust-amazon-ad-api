use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbBenchmarkRequest {
    pub targets: Option<Vec<serde_json::Value>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub match_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbBenchmarkResult {
    pub keyword: Option<String>,
    pub match_type: Option<String>,
    pub daily_budget: Option<f64>,
    pub bid: Option<f64>,
    pub impressions: Option<i64>,
    pub clicks: Option<i64>,
    pub cost: Option<f64>,
}
