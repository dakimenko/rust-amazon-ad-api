use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct Profile {
    pub profile_id: Option<i64>,
    pub country_code: Option<String>,
    pub currency_code: Option<String>,
    pub daily_budget: Option<f64>,
    pub timezone: Option<String>,
    pub account_info: Option<AccountInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct AccountInfo {
    pub marketplace_string_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub name: Option<String>,
    pub valid_payment_method: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct RegisterProfileRequest {
    pub country_code: Option<String>,
}
