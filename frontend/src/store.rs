use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use wherego::{Destination, Score};

pub const DEFAULT_USERNAME: &str = "edit me";

pub fn fetch_dests_scores() {
    let dest_dispatch = Dispatch::<Destinations>::new();
    let scores_dispatch = Dispatch::<Scores>::new();
    yew::platform::spawn_local(async move {
        let sent = reqwest::get("http://127.0.0.1:3030/api/destinations")
            .await
            .unwrap();
        let received = sent.json().await.unwrap();
        dest_dispatch.set(Destinations { value: received });
        let sent = reqwest::get("http://127.0.0.1:3030/api/scores")
            .await
            .unwrap();
        let received = sent.json().await.unwrap();
        scores_dispatch.set(Scores { value: received });
    });
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct NewDestination {
    pub value: Destination,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct Destinations {
    pub value: Vec<Destination>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct Scores {
    pub value: Vec<Score>,
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
