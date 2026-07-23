use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbSnapshotRequest {
    pub record_type: Option<String>,
    pub state_filter: Option<String>,
    pub campaign_id_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbSnapshotResponse {
    pub snapshot_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
    pub location: Option<String>,
    pub file_size: Option<i64>,
    pub expiration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default)]
pub struct SbSnapshotStatusResponse {
    pub snapshot_id: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<String>,
}
