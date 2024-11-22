//! All internally defined data structures used in the application, this includes in memory data and data stored in SurrealDB.

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod access_token;
pub mod datetime;
pub mod refresh_token;
pub mod session;
pub mod thing;
pub mod user_info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: Thing,
}
