use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AttributionAdvertiser {
    pub advertiser_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AttributionReportRequest {
    pub advertiser_id: Option<String>,
    pub report_type: Option<String>,
    pub date_range: Option<String>,
}
