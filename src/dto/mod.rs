#![allow(unused_imports)]

//! All items in this module are DTOs that are used to transfer data between
//! the server and the client. A DTO is not meant to be used as an internal model and therefore is separate from the models module

pub(crate) mod access_token_request;
pub(crate) mod file_upload_form;
pub(crate) mod oauth_callback;
pub(crate) mod token;
pub(crate) mod user_info;
pub(crate) mod user_registration_request;
pub(crate) mod user_update_request;

pub(crate) use {
    access_token_request::*, oauth_callback::*, token::*, user_info::*,
    user_registration_request::*,
};

use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};
use std::string::String;

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

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct PaginationResponse {
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
    pub(crate) total: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct PaginationRequest {
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CountResponse {
    pub(crate) count: u64,
}
