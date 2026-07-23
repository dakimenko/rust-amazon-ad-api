use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbNegativeKeywordState { Enabled, Paused, Archived }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SbNegativeMatchType { NegativeExact, NegativePhrase, NegativeBroad }

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbNegativeKeyword {
    pub keyword_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<SbNegativeKeywordState>,
    pub keyword_text: Option<String>,
    pub match_type: Option<SbNegativeMatchType>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbNegativeKeywordResponse {
    pub keyword_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbListNegativeKeywordsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub keyword_text: Option<String>,
    pub match_type_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
}
