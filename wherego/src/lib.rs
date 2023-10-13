use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    pub name: String,
    pub description: String,
    pub id: i64,
}

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub username: String,
    pub dest_id: i64,
    pub score: i64,
}
