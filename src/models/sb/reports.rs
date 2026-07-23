use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbReportRequest {
    pub report_date: Option<String>,
    pub record_type: Option<String>,
    pub segment: Option<serde_json::Value>,
    pub metrics: Option<String>,
    pub creative_type: Option<String>,
    pub tactic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbReportResponse {
    pub report_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
    pub location: Option<String>,
    pub file_size: Option<i64>,
    pub expiration: Option<i64>,
    pub record_type: Option<String>,
    pub url: Option<String>,
}
