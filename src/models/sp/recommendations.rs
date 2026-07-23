use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct RankedKeywordRecommendation {
    pub keyword_id: Option<String>,
    pub keyword_text: Option<String>,
    pub match_type: Option<String>,
    pub bid: Option<f64>,
    pub rank: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SuggestedKeyword {
    pub keyword_text: Option<String>,
    pub match_type: Option<super::keywords::MatchType>,
    pub bid: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SuggestedKeywordsRequest {
    pub ad_group_id: Option<String>,
    pub max_num_suggestions: Option<i32>,
    pub ad_asins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ProductRecommendationRequest {
    pub ad_group_id: Option<String>,
    pub campaign_id: Option<String>,
    pub max_recommendations: Option<i32>,
}
