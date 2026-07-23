use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ReportRequestV3 {
    pub record_type: Option<String>,
    pub report_date: Option<String>,
    pub metrics: Option<String>,
    pub configuration: Option<ReportConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ReportConfiguration {
    pub ad_product: Option<String>,
    pub group_by: Option<Vec<String>>,
    pub columns: Option<Vec<String>>,
    pub report_type_id: Option<String>,
    pub time_unit: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ReportResponseV3 {
    pub report_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
    pub location: Option<String>,
    pub file_size: Option<i64>,
    pub expiration: Option<i64>,
    pub url: Option<String>,
}
