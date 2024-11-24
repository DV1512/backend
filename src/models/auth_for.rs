use crate::models::datetime::Datetime;
use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub struct AuthForRelation {
    pub(crate) id: Thing,
    created_at: Datetime,
    updated_at: Datetime,
    password: Option<String>,
    providers: Vec<Thing>,
}
