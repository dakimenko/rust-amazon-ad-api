use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PortfolioState { Enabled, Paused, Archived }

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct Portfolio {
    pub portfolio_id: Option<i64>,
    pub name: Option<String>,
    pub budget: Option<Budget>,
    pub state: Option<PortfolioState>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct Budget {
    pub amount: Option<f64>,
    pub end_date: Option<String>,
    pub policy: Option<String>,
    pub start_date: Option<String>,
}
