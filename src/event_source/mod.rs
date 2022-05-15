pub mod parser;
pub mod stream;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{core::session::URLParser, TypeState};

pub enum URLParameter {
    Types,
    CloseAfter,
    Ping,
}

impl URLParser for URLParameter {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "types" => Some(URLParameter::Types),
            "closeafter" => Some(URLParameter::CloseAfter),
            "ping" => Some(URLParameter::Ping),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Changes {
    changes: HashMap<String, HashMap<TypeState, String>>,
}

impl Changes {
    pub fn account_changes(&mut self, account_id: &str) -> Option<HashMap<TypeState, String>> {
        self.changes.remove(account_id)
    }

    pub fn changed_accounts(&self) -> impl Iterator<Item = &String> {
        self.changes.keys()
    }

    pub fn into_innter(self) -> HashMap<String, HashMap<TypeState, String>> {
        self.changes
    }
}
