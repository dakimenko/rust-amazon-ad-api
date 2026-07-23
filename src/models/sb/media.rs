use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbMediaUploadResponse {
    pub media_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
    pub location: Option<String>,
}
