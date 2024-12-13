//! All internally defined data structures used in the application, this includes in memory data and data stored in SurrealDB.
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod access_token;
pub mod auth_for;
pub mod datetime;
pub mod embeddings;
pub mod file_metadata;
pub mod refresh_token;
pub mod session;
pub mod thing;
pub mod user_info;

pub(crate) use access_token::*;
pub(crate) use auth_for::*;
pub(crate) use embeddings::*;
pub(crate) use file_metadata::*;
pub(crate) use refresh_token::*;
pub(crate) use session::*;
pub(crate) use thing::*;
pub(crate) use user_info::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: Thing,
}
