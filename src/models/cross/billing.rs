use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct BillingInfo {
    pub billing_id: Option<String>,
    pub billing_period: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
}
