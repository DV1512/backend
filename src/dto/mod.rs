//! All items in this module are DTOs that are used to transfer data between
//! the server and the client. A DTO is not meant to be used as an internal model and therefore is separate from the models module

pub(crate) mod access_token_request;
pub(crate) mod user_info;

use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};
use std::string::String;

pub(crate) use user_info::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct IdDTO {
    pub id: String,
    pub tb: String,
}

impl From<Thing> for IdDTO {
    fn from(thing: Thing) -> Self {
        IdDTO {
            id: thing.id.to_string(),
            tb: thing.tb.clone(),
        }
    }
}

impl From<Option<Thing>> for IdDTO {
    fn from(thing: Option<Thing>) -> Self {
        match thing {
            Some(thing) => thing.into(),
            None => IdDTO::default(),
        }
    }
}
