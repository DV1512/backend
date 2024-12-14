use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

use super::thing::Thing;

/// Represents an entry from the MITRE ATT&CK database
#[derive(Serialize, Deserialize, ToSchema)]
pub struct MITREEntry {
    pub mitre_id: String,
    pub mitre_name: String,
    pub mitre_description: String,
    pub mitre_url: String,
}

#[allow(dead_code)]
#[derive(ToResponse)]
pub struct MITREEntries(pub Vec<MITREEntry>);

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Threat,
    Mitigation,
}

/// Represents an entry in the 'mitigation' or
/// 'threat' table in the database.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Entry {
    pub id: Option<Thing>,
    pub similarity: Option<f32>,
    pub embedding: Option<Vec<f32>>,

    #[serde(flatten)]
    mitre: MITREEntry,
}
