use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamSubscription {
    pub subscription_id: Option<String>,
    pub topic: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamSubscriptionResponse {
    pub subscription_id: Option<String>,
    pub status: Option<String>,
    pub data: Option<serde_json::Value>,
}
