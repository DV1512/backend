pub(crate) mod user_info;

use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};
use std::string::String;

pub(crate) use user_info::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            None => IdDTO {
                id: String::new(),
                tb: String::new(),
            },
        }
    }
}
