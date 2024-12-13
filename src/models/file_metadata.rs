use super::{datetime::Datetime, thing::Thing};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

/// A type representing the metadata of an uploaded file.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct FileMetadata {
    pub id: Thing,
    /// The original filename of the uploaded file.
    /// For security reasons, uploaded files are
    /// persisted to disk with a filename equal
    /// to its database record ID.
    pub filename: String,
    /// The datetime when the file was uploaded.
    pub created_at: Datetime,
}

#[allow(dead_code)]
#[derive(ToResponse)]
pub struct FileMetadataMultiple(pub Vec<FileMetadata>);
