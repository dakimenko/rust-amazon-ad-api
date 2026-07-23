use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NegativeMatchType {
    NegativeExact,
    NegativePhrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NegativeKeywordState {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct NegativeKeyword {
    pub keyword_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<NegativeKeywordState>,
    pub keyword_text: Option<String>,
    pub match_type: Option<NegativeMatchType>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct NegativeKeywordResponse {
    pub keyword_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListNegativeKeywordsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub keyword_id_filter: Option<String>,
    pub match_type_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteNegativeKeywordsRequest {
    pub keyword_id_filter: super::campaigns::ObjectIdFilter,
}
