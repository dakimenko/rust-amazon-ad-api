use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetingState {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetingExpression {
    #[serde(rename = "type")]
    ExpressionType(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct TargetingClause {
    pub target_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<TargetingState>,
    pub expression: Option<Vec<TargetingExpression>>,
    pub expression_type: Option<String>,
    pub bid: Option<f64>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListTargetsRequest {
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
    pub ad_group_id_filter: Option<String>,
    pub target_id_filter: Option<String>,
    pub max_results: Option<i32>,
    pub next_token: Option<String>,
    pub include_extended_data_fields: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteTargetsRequest {
    pub target_id_filter: super::campaigns::ObjectIdFilter,
}

// ── Theme Targeting (SP v3) ────────────────────────────────────

/// Theme type values for SP v3 `spThemeTargeting`.
///
/// Amazon Advertising SP v3 supports theme-based targeting where
/// advertisers select content-relevant themes rather than explicit keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpThemeType {
    /// Target users browsing seasonal sales events.
    SeasonalEvent,
    /// Target users by entertainment genres or topics.
    EntertainmentGenre,
    /// Target users engaged with specific lifestyle content.
    Lifestyle,
    /// Target users interested in a product category.
    ProductCategory,
}

/// A single SP v3 theme targeting expression.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SpThemeTargetingExpression {
    /// Theme type for this targeting expression.
    pub theme_type: Option<SpThemeType>,
    /// Theme value identifier (Amazon-defined theme ID or name).
    pub theme_value: Option<String>,
}

/// Full SP v3 theme targeting clause for a targeting clause payload.
///
/// Use `spThemeTargeting` in the SP v3 targets API to target audiences
/// browsing theme-relevant content (e.g. holiday shopping, specific lifestyle).
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SpThemeTargetingClause {
    pub target_id: Option<String>,
    pub campaign_id: Option<String>,
    pub ad_group_id: Option<String>,
    pub state: Option<TargetingState>,
    /// Theme-based targeting expressions for this clause.
    pub sp_theme_targeting: Option<Vec<SpThemeTargetingExpression>>,
    pub bid: Option<f64>,
    pub creation_date: Option<i64>,
    pub last_updated_date: Option<i64>,
    pub serving_status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sp_theme_targeting_clause_serialization() {
        let clause = SpThemeTargetingClauseBuilder::default()
            .campaign_id("camp-001")
            .ad_group_id("adg-001")
            .state(TargetingState::Enabled)
            .sp_theme_targeting(vec![SpThemeTargetingExpressionBuilder::default()
                .theme_type(SpThemeType::SeasonalEvent)
                .theme_value("HOLIDAY_SHOPPING_2024")
                .build()
                .unwrap()])
            .bid(1.50_f64)
            .build()
            .unwrap();

        let json = serde_json::to_value(&clause).unwrap();
        assert_eq!(json["state"], "enabled");
        assert_eq!(json["bid"], 1.50);
        assert_eq!(
            json["spThemeTargeting"][0]["themeValue"],
            "HOLIDAY_SHOPPING_2024"
        );
    }
}
