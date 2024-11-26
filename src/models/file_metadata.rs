use super::{datetime::Datetime, thing::Thing};
use serde::{Deserialize, Serialize};

/// A type representing the metadata of an uploaded file.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
