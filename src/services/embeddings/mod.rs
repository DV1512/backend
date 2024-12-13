use serde::{Deserialize, Serialize};

use crate::models::thing::Thing;

pub mod add;
pub mod search;

/// Represents an entry from the MITRE ATT&CK database
#[derive(Serialize, Deserialize)]
pub struct MITREEntry {
    pub mitre_id: String,
    pub mitre_name: String,
    pub mitre_description: String,
    pub mitre_url: String,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Threat,
    Mitigation,
}

impl From<EntryType> for String {
    fn from(t: EntryType) -> Self {
        let s = match t {
            EntryType::Threat => "threat",
            EntryType::Mitigation => "mitigation",
        };
        String::from(s)
    }
}
