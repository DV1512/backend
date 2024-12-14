use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::Entry;
use crate::models::EntryType;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddEmbeddingsRequest {
    #[serde(rename = "type")]
    pub entry_type: EntryType,

    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SearchEmbeddingsRequest {
    #[serde(rename = "type")]
    pub entry_type: EntryType,

    pub embedding: Vec<f32>,
    pub num_neighbors: u32,
}
