use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DspReportRequest {
    pub report_type: Option<String>,
    pub report_date: Option<String>,
    pub metrics: Option<String>,
    pub dimensions: Option<Vec<String>>,
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DspReportResponse {
    pub report_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
    pub location: Option<String>,
    pub file_size: Option<i64>,
    pub expiration: Option<i64>,
}
