use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct BidRecommendation {
    pub keyword_id: Option<String>,
    pub target_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub campaign_id: Option<String>,
    pub suggested_bid: Option<SuggestedBid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SuggestedBid {
    pub suggested: Option<f64>,
    pub range_start: Option<f64>,
    pub range_end: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ThemeBasedBidRecommendation {
    pub campaign_id: Option<String>,
    pub theme: Option<String>,
    pub suggested_bid: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct BidRecommendationsRequest {
    pub ad_group_id: Option<String>,
    pub keyword_id: Option<String>,
    pub target_id: Option<String>,
}
