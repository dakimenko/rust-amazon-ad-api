use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Stream dataset identifier for Amazon Marketing Stream (AMS v2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StreamDataSet {
    /// Sponsored Products near real-time performance (impressions, clicks, cost).
    SpPerformance,
    /// Sponsored Brands near real-time performance.
    SbPerformance,
    /// Sponsored Display near real-time performance.
    SdPerformance,
    /// Sponsored Products near real-time traffic (detailed click/impression streams).
    SpTraffic,
    /// Real-time budget usage and notifications.
    BudgetUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamSubscription {
    pub subscription_id: Option<String>,
    pub data_set_id: Option<StreamDataSet>,
    pub topic: Option<String>,
    pub status: Option<String>,
    pub destination_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamSubscriptionResponse {
    pub subscription_id: Option<String>,
    pub status: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// AMS v2 Performance Message Payload (near real-time impressions, clicks, spend).
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamPerformanceRecord {
    pub dataset: Option<String>,
    pub marketplace_id: Option<String>,
    pub profile_id: Option<i64>,
    pub campaign_id: Option<i64>,
    pub ad_group_id: Option<i64>,
    pub ad_id: Option<i64>,
    pub keyword_id: Option<i64>,
    pub target_id: Option<i64>,
    pub impressions: Option<i64>,
    pub clicks: Option<i64>,
    pub cost: Option<f64>,
    pub timestamp: Option<String>,
}

/// AMS v2 Budget Usage Message Payload (real-time budget updates).
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct StreamBudgetUsageRecord {
    pub dataset: Option<String>,
    pub profile_id: Option<i64>,
    pub campaign_id: Option<i64>,
    pub budget: Option<f64>,
    pub budget_usage_percentage: Option<f64>,
    pub usage_updated_timestamp: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_dataset_enum_serialization() {
        let ds = StreamDataSet::SpPerformance;
        let json = serde_json::to_string(&ds).unwrap();
        assert_eq!(json, "\"sp-performance\"");
    }

    #[test]
    fn test_stream_performance_record_deserialization() {
        let json_data = r#"{
            "dataset": "sp-performance",
            "marketplaceId": "ATVPDKIKX0DER",
            "profileId": 123456789,
            "campaignId": 987654321,
            "impressions": 150,
            "clicks": 12,
            "cost": 14.50,
            "timestamp": "2024-11-15T10:00:00Z"
        }"#;

        let record: StreamPerformanceRecord = serde_json::from_str(json_data).unwrap();
        assert_eq!(record.dataset.as_deref(), Some("sp-performance"));
        assert_eq!(record.profile_id, Some(123456789));
        assert_eq!(record.impressions, Some(150));
        assert_eq!(record.clicks, Some(12));
        assert_eq!(record.cost, Some(14.50));
    }
}
