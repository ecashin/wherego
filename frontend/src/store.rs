use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use wherego::{Destination, Score};

use crate::full_url;

pub const DEFAULT_USERNAME: &str = "edit me";

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Store)]
pub struct SelectedDestinationId {
    pub value: i64,
}

impl Default for SelectedDestinationId {
    fn default() -> Self {
        Self { value: -1 }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Store)]
pub struct OverDestinationId {
    pub value: i64,
}

impl Default for OverDestinationId {
    fn default() -> Self {
        Self { value: -1 }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Store)]
pub struct BaseUrl {
    pub value: String,
}

impl Default for BaseUrl {
    // https://github.com/seanmonstar/reqwest/issues/988#issuecomment-1047403159
    fn default() -> Self {
        Self {
            value: web_sys::window().unwrap().origin().to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct NegotiationResults {
    pub value: Option<Vec<Destination>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct DestBeingEdited {
    pub value: Option<Destination>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default, Eq, Serialize, Store)]
pub struct CheckedUsernames {
    pub value: Vec<(String, bool)>,
}

pub fn fetch_dests_scores() {
    let dest_dispatch = Dispatch::<Destinations>::new();
    let scores_dispatch = Dispatch::<Scores>::new();
    let checked_usernames_dispatch = Dispatch::<CheckedUsernames>::new();
    yew::platform::spawn_local(async move {
        let sent = reqwest::get(full_url("/api/destinations")).await.unwrap();
        let received = sent.json().await.unwrap();
        dest_dispatch.set(Destinations { value: received });
        let sent = reqwest::get(full_url("/api/scores")).await.unwrap();
        let received: Vec<Score> = sent.json().await.unwrap();
        let mut usernames: Vec<_> = received
            .iter()
            .map(|score| score.username.clone())
            .collect();
        usernames.sort();
        usernames.dedup();
        scores_dispatch.set(Scores { value: received });
        checked_usernames_dispatch.set(CheckedUsernames {
            value: usernames.into_iter().map(|u| (u, false)).collect(),
        });
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
