use serde::{Deserialize, Serialize};

use super::thing::Thing;

/// Represents an entry from the MITRE ATT&CK database
#[derive(Serialize, Deserialize)]
pub struct MITREEntry {
    pub mitre_id: String,
    pub mitre_name: String,
    pub mitre_description: String,
    pub mitre_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Threat,
    Mitigation,
}

/// Represents an entry in the 'mitigation' or
/// 'threat' table in the database.
#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: Option<Thing>,
    pub similarity: Option<f32>,
    pub embedding: Option<Vec<f32>>,

    #[serde(flatten)]
    mitre: MITREEntry,
}
