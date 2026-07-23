use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use derive_builder::Builder;

/// Targeting type for campaigns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TargetingType {
    Auto,
    Manual,
}

/// Campaign state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CampaignState {
    Enabled,
    Paused,
    Archived,
}

/// Daily budget.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DailyBudget {
    #[serde(rename = "budgetType")]
    pub budget_type: String, // "DAILY"
    pub budget: f64,
}

/// Dynamic bidding configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DynamicBidding {
    pub placement_bidding: Option<Vec<PlacementBidding>>,
    pub strategy: Option<String>, // LEGACY_FOR_SALES, AUTO_FOR_SALES, MANUAL, RULE_BASED
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct PlacementBidding {
    pub placement: Option<String>,
    pub percentage: Option<f64>,
}

/// Sponsored Products Campaign.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct Campaign {
    pub campaign_id: Option<String>,
    pub name: Option<String>,
    pub state: Option<CampaignState>,
    pub targeting_type: Option<TargetingType>,
    pub budget: Option<DailyBudget>,
    pub dynamic_bidding: Option<DynamicBidding>,
    pub portfolio_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub tags: Option<HashMap<String, String>>,
    /// Extra fields returned in extended listings.
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

/// Request body for listing campaigns.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListCampaignsRequest {
    pub state_filter: Option<String>,
    pub name_filter: Option<NameFilter>,
    pub portfolio_id_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_format_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct NameFilter {
    pub query_term_match_type: Option<String>,
    pub query: Option<String>,
}

/// Request body for deleting campaigns.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCampaignsRequest {
    pub campaign_id_filter: ObjectIdFilter,
}

/// Generic filter by object IDs used across delete operations.
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ObjectIdFilter {
    pub include: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_campaign() {
        let json = r#"{
            "campaignId": "1234567890",
            "name": "Test Campaign",
            "state": "ENABLED",
            "targetingType": "MANUAL",
            "budget": { "budgetType": "DAILY", "budget": 10.0 },
            "startDate": "20240101",
            "portfolioId": 123
        }"#;
        let campaign: Campaign = serde_json::from_str(json).unwrap();
        assert_eq!(campaign.campaign_id, Some("1234567890".to_string()));
        assert_eq!(campaign.name, Some("Test Campaign".to_string()));
        assert_eq!(campaign.state, Some(CampaignState::Enabled));
        assert_eq!(campaign.targeting_type, Some(TargetingType::Manual));
        let budget = campaign.budget.unwrap();
        assert_eq!(budget.budget_type, "DAILY");
        assert!((budget.budget - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_list_request_builder() {
        let req = ListCampaignsRequestBuilder::default()
            .state_filter("enabled".to_string())
            .max_results(50)
            .build()
            .unwrap();
        assert_eq!(req.state_filter, Some("enabled".to_string()));
        assert_eq!(req.max_results, Some(50));
    }

    #[test]
    fn test_delete_request_builder() {
        let filter = ObjectIdFilterBuilder::default()
            .include(vec!["abc123".to_string()])
            .build()
            .unwrap();
        let req = DeleteCampaignsRequestBuilder::default()
            .campaign_id_filter(filter)
            .build()
            .unwrap();
        assert_eq!(req.campaign_id_filter.include, vec!["abc123"]);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct CampaignResponse {
    pub campaign_id: Option<String>,
    pub code: Option<String>,
    pub details: Option<String>,
}
