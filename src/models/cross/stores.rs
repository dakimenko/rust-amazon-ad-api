use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct Store {
    pub store_id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StoreAsset {
    pub asset_id: Option<String>,
    pub store_id: Option<String>,
    pub asset_type: Option<String>,
    pub url: Option<String>,
}
