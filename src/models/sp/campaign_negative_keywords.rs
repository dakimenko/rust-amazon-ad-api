use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct CampaignNegativeKeyword {
    pub keyword_id: Option<String>,
    pub campaign_id: Option<String>,
    pub state: Option<super::negative_keywords::NegativeKeywordState>,
    pub keyword_text: Option<String>,
    pub match_type: Option<super::negative_keywords::NegativeMatchType>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct CampaignNegativeKeywordResponse {
    pub keyword_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListCampaignNegativeKeywordsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub keyword_id_filter: Option<String>,
    pub match_type_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCampaignNegativeKeywordsRequest {
    pub keyword_id_filter: super::campaigns::ObjectIdFilter,
}
