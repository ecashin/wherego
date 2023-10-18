use std::rc::Rc;

use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
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

fn sort_by_score(username: &str, dests: Vec<Destination>, scores: &Vec<Score>) -> Vec<Destination> {
    let mut dest_scores = dests
        .iter()
        .map(|d| {
            let score = scores
                .iter()
                .find_map(|s| {
                    if s.dest_id == d.id && s.username == username {
                        Some(s.score)
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            (d, score)
        })
        .collect::<Vec<_>>();
    log!(JsValue::from(&format!("dest_scores:{dest_scores:?}")));
    dest_scores.sort_by_key(|(_d, s)| s * -1);
    log!(JsValue::from(&format!("dest_scores:{dest_scores:?}")));
    dest_scores
        .into_iter()
        .map(|(d, _s)| d.clone())
        .collect::<Vec<_>>()
}

pub fn fetch_dests_scores() {
    let dest_dispatch = Dispatch::<Destinations>::new();
    let scores_dispatch = Dispatch::<Scores>::new();
    let checked_usernames_dispatch = Dispatch::<CheckedUsernames>::new();
    yew::platform::spawn_local(async move {
        let sent = reqwest::get(full_url("/api/destinations")).await.unwrap();
        let dests = sent.json().await.unwrap();
        let sent = reqwest::get(full_url("/api/scores")).await.unwrap();
        let scores: Vec<Score> = sent.json().await.unwrap();
        let username = &Dispatch::<Username>::new().get().value;
        let mut usernames: Vec<_> = scores.iter().map(|score| score.username.clone()).collect();
        usernames.sort();
        usernames.dedup();
        dest_dispatch.set(Destinations {
            value: sort_by_score(username, dests, &scores),
        });
        scores_dispatch.set(Scores { value: scores });
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
#[store(storage = "local", listener(UsernameListener), storage_tab_sync)]
pub struct Username {
    pub value: String,
}

/*
impl Store for Username {
    fn new() -> Self {
        init_listener(UsernameListener);
        Self {
            value: storage::load(storage::Area::Local)
                .ok()
                .flatten()
                .unwrap_or_else(|| {
                    log!(JsValue::from("Username::new lambda"));
                    DEFAULT_USERNAME.to_string()
                }),
        }
    }

    fn should_notify(&self, other: &Self) -> bool {
        self != other
    }
}
*/

impl Default for Username {
    fn default() -> Self {
        log!(JsValue::from("Username::default"));
        Self {
            value: DEFAULT_USERNAME.to_string(),
        }
    }
}

struct UsernameListener;
impl Listener for UsernameListener {
    type Store = Username;

    fn on_change(&mut self, _state: Rc<Self::Store>) {
        fetch_dests_scores();
    }
}
