use serde::{Deserialize, Serialize};
use crate::models::datetime::Datetime;
use crate::models::thing::Thing;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub struct AuthForRelation {
    pub(crate) id: Thing,
    created_at: Datetime,
    updated_at: Datetime,
    password: Option<String>,
    providers: Vec<Thing>,
}