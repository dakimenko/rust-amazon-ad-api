use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdBrandSafetyDenyList {
    pub domain_list: Option<Vec<SdDenyListDomain>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SdDenyListDomain {
    pub domain: Option<String>,
    pub state: Option<String>,
}
