use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbCreative {
    pub creative_id: Option<String>,
    pub creative_type: Option<super::campaigns::SbCreativeType>,
    pub brand_logo_asset_id: Option<String>,
    pub headline: Option<String>,
    pub video_media_ids: Option<Vec<String>>,
    pub asins: Option<Vec<String>>,
    pub landing_page_url: Option<String>,
    pub brand_name: Option<String>,
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbCreativeResponse {
    pub creative_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbListCreativesRequest {
    pub creative_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
}
