use std::fmt::Display;

use crate::models::EntryType;

pub mod add;
pub mod search;

impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Threat => write!(f, "threat"),
            EntryType::Mitigation => write!(f, "mitigation"),
        }
    }
}
