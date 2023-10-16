use std::rc::Rc;

use gloo_console::log;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

use wherego::{Destination, Score};

use crate::{full_url, store};

#[function_component]
pub fn NegotiationResultsC() -> Html {
    let (results, dispatch) = use_store::<store::NegotiationResults>();
    let dismiss = {
        let onclick = dispatch.reduce_mut_callback_with(|n, _| {
            n.value = None;
        });
        html! {
            <button {onclick}>{"Dismiss"}</button>
        }
    };
    let results_html = results
        .value
        .as_ref()
        .unwrap()
        .iter()
        .map(|d| {
            html! {
                <DestinationC dest={d.clone()} is_read_only={true} />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <div>
            {dismiss}
            <table class="is-striped">
                <tr><th>{"Destination"}</th><th>{"Description"}</th><th>{"My Score"}</th></tr>
                {results_html}
            </table>
        </div>
    }
}

// https://stackoverflow.com/questions/69764050/how-to-get-the-indices-that-would-sort-a-vec/69764256#69764256
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

#[function_component]
pub fn UserSelectC() -> Html {
    let (users, dispatch) = use_store::<store::CheckedUsernames>();
    let (_, negotiation_dispatch) = use_store::<store::NegotiationResults>();
    let (scores, _) = use_store::<store::Scores>();
    let (dests, _) = use_store::<store::Destinations>();
    let checkboxes = users
        .value
        .iter()
        .enumerate()
        .map(|(i, (username, is_checked))| {
            let id = format!("checkbox{i}");
            let onclick = dispatch.reduce_mut_callback_with(move |users, _| {
                users.value[i].1 = !users.value[i].1;
            });
            html! {
                <>
                    <input type={"checkbox"} id={id.clone()} checked={*is_checked} {onclick} />
                    <label for={id}>{username}</label>
                </>
            }
        })
        .collect::<Vec<_>>();
    let negotiate = {
        let dests = dests.value.clone();
        let onclick =
            negotiation_dispatch.reduce_mut_callback_with(move |negotiation_results, _| {
                let participants = users
                    .value
                    .iter()
                    .filter_map(|(u, is_checked)| if *is_checked { Some(u.clone()) } else { None })
                    .collect::<Vec<_>>();
                // collect one series of canonical scores per checked user
                let scores_per_p = participants
                    .iter()
                    .map(|u| {
                        let mut s_p = vec![]; // participant's scores for all destinations
                        for d in &dests {
                            let s_d = scores
                                .value
                                .iter()
                                .find(|&score| score.dest_id == d.id && score.username == *u);
                            s_p.push(if let Some(score) = s_d {
                                score.score
                            } else {
                                0
                            });
                        }
                        argsort(&s_p)
                    })
                    .collect::<Vec<_>>();
                // sum canonical scores for each destination
                let mut totals = if scores_per_p.is_empty() {
                    vec![]
                } else {
                    vec![0; dests.len()]
                };
                for s_p in &scores_per_p {
                    assert_eq!(s_p.len(), dests.len());
                    for i in 0..dests.len() {
                        totals[i] += s_p[i];
                    }
                }
                let mut results = dests.iter().zip(totals.into_iter()).collect::<Vec<_>>();
                results.sort_by_key(|(_d, count)| -(*count as i64));
                negotiation_results.value = Some(
                    results
                        .into_iter()
                        .map(|(d, _s)| d.clone())
                        .collect::<Vec<_>>(),
                );
            });
        html! {
            <button {onclick}>{"Negotiate"}</button>
        }
    };
    html! {
        <div class={"has-background-grey-light has-text-light"}>
            {checkboxes}
            {negotiate}
        </div>
    }
}

#[function_component]
pub fn DestEditC() -> Html {
    let (editing_dest, editing_dest_dispatch) = use_store::<store::DestBeingEdited>();
    let name = {
        let oninput = editing_dest_dispatch.reduce_mut_callback_with(|d, e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            d.value.as_mut().unwrap().name = input.value();
        });
        html! {
            <input value={editing_dest.value.as_ref().unwrap().name.clone()} {oninput} />
        }
    };
    let desc = {
        let oninput = editing_dest_dispatch.reduce_mut_callback_with(|d, e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            d.value.as_mut().unwrap().description = input.value();
        });
        html! {
            <textarea value={editing_dest.value.as_ref().unwrap().description.clone()} {oninput} />
        }
    };
    let cancel = {
        let onclick = editing_dest_dispatch.reduce_mut_callback_with(|d, _| {
            d.value = None;
        });
        html! {
            <button {onclick}>{"Cancel"}</button>
        }
    };
    let submit = {
        let onclick = editing_dest_dispatch.reduce_mut_callback_with(|d, _| {
            put_destination("edited destination", d.value.as_ref().unwrap());
            d.value = None;
        });
        html! {
            <button {onclick}>{"Submit"}</button>
        }
    };
    html! {
        <div>
            {name}
            {desc}
            {cancel}
            {submit}
        </div>
    }
}

#[function_component]
pub fn ScoresC() -> Html {
    let (destinations, _dests_dispatch) = use_store::<store::Destinations>();
    let dests_html = destinations
        .value
        .iter()
        .map(|d| {
            html! {
                <DestinationC dest={d.clone()} />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <table class={"is-striped"}>
            <tr><th>{"New Destination"}</th><th>{"Description"}</th><th></th><th></th></tr>
            <NewDestinationC />
            <tr><th>{"Destination"}</th><th>{"Description"}</th><th></th><th>{"Score"}</th></tr>
            {dests_html}
        </table>
    }
}

fn put_destination(msg: &str, d: &Destination) {
    log!(JsValue::from(msg));
    let d = d.clone();
    yew::platform::spawn_local(async move {
        reqwest::Client::new()
            .put(full_url("/api/destination"))
            .json(&d)
            .send()
            .await
            .unwrap();
        store::fetch_dests_scores();
    });
}

fn post_new_destination(msg: &str, d: &Destination) {
    log!(JsValue::from(msg));
    let d = d.clone();
    yew::platform::spawn_local(async move {
        reqwest::Client::new()
            .post(full_url("/api/destinations"))
            .json(&d)
            .send()
            .await
            .unwrap();
        store::fetch_dests_scores();
    });
}

fn post_score(msg: &str, s: &Score) {
    log!(JsValue::from(msg));
    let s = s.clone();
    yew::platform::spawn_local(async move {
        reqwest::Client::new()
            .post(full_url("/api/scores"))
            .json(&s)
            .send()
            .await
            .unwrap();
        // in case it's a new username or someone else stored scores ...
        store::fetch_dests_scores();
    });
}

#[function_component]
pub fn NewDestinationC() -> Html {
    let (new_dest, dispatch) = use_store::<store::NewDestination>();
    let oninput_name = dispatch.reduce_mut_callback_with(|d, e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
        d.value.name = input.value();
        d.value.id = -1;
    });
    let oninput_desc = dispatch.reduce_mut_callback_with(|d, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
        d.value.description = input.value();
        d.value.id = -1;
    });
    let onclick = dispatch.reduce_callback_with(|d, _| {
        post_new_destination("sending new destination to database", &d.value);
        Rc::new(store::NewDestination::default())
    });
    html! {
        <tr class={"has-text-primary-dark has-background-light"}>
            <td><input value={new_dest.value.name.to_string()} oninput={oninput_name} /></td>
            <td><textarea value={new_dest.value.description.to_string()} oninput={oninput_desc} /></td>
            <td><button {onclick}>{"create"}</button></td>
            <td></td>
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct DestinationCProps {
    pub dest: Destination,
    #[prop_or_default]
    pub is_read_only: bool,
}

#[function_component]
pub fn DestinationC(props: &DestinationCProps) -> Html {
    let (username, _) = use_store::<store::Username>();
    let (scores, scores_dispatch) = use_store::<store::Scores>();
    let (_editing_dest, editing_dest_dispatch) = use_store::<store::DestBeingEdited>();
    let dest_id = props.dest.id;
    let dest_score = {
        let mut score = 0;
        for s in &scores.value {
            if s.dest_id == dest_id && s.username == username.value {
                score = s.score;
            }
        }
        score
    };
    let edit_button = if props.is_read_only {
        html! {}
    } else {
        let dest = props.dest.clone();
        let onclick = editing_dest_dispatch.reduce_mut_callback_with(move |d, _| {
            d.value = Some(dest.clone());
        });
        html! {
            <button {onclick}>{"Edit"}</button>
        }
    };
    let score_html = if !props.is_read_only && username.value != store::DEFAULT_USERNAME {
        let oninput = scores_dispatch.reduce_mut_callback_with(move |scores, e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            let mut changed = false;
            if let Ok(num) = input.value().parse::<i64>() {
                for s in scores.value.iter_mut() {
                    if s.dest_id == dest_id && s.username == username.value {
                        s.score = num;
                        changed = true;
                        post_score("update existing score", s);
                    }
                }
                if !changed {
                    scores.value.push(Score {
                        username: username.value.clone(),
                        dest_id,
                        score: num,
                    });
                    post_score("adding new score", &scores.value[scores.value.len() - 1]);
                }
            }
        });
        html! {
            <input value={dest_score.to_string()} {oninput} />
        }
    } else {
        html! {
            <span>{dest_score.to_string()}</span>
        }
    };
    html! {
        <tr>
            <td>{props.dest.name.clone()}</td>
            <td>{props.dest.description.clone()}</td>
            <td>{edit_button}</td>
            <td>{score_html}</td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub heading: String,
    pub oninput: Callback<InputEvent>,
    pub text: String,
}

#[function_component]
pub fn Text(props: &TextProps) -> Html {
    //        <b style={"padding: 0 1rem 0 0.3rem"}>{props.heading.clone()}</b>
    html! {
        <div>
            <label for={"username-input"}>{props.heading.clone()}</label>
            <input id={"username-input"} value={props.text.clone()} oninput={props.oninput.clone()} />
        </div>
    }
}

#[function_component]
pub fn Username() -> Html {
    let (username, dispatch) = use_store::<store::Username>();
    let oninput = dispatch.reduce_mut_callback_with(|u, e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
        u.value = input.value();
    });
    html! {
        <div class={"has-text-light has-background-dark"}>
            <Text heading={"username"} text={username.value.clone()} {oninput} />
        </div>
    }
}
