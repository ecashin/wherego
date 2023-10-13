use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use wherego::Destination;

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct Destinations {
    pub value: Vec<Destination>,
}
