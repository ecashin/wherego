use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use wherego::Destination;

const DEFAULT_USERNAME: &str = "edit me";

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct Destinations {
    pub value: Vec<Destination>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct Username {
    pub value: String,
}

impl Default for Username {
    fn default() -> Self {
        Self {
            value: DEFAULT_USERNAME.to_string(),
        }
    }
}
