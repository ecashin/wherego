use std::rc::Rc;

use gloo_console::log;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

use wherego::{Destination, Score};

use crate::store;

#[function_component]
pub fn UserSelectC() -> Html {
    let (users, dispatch) = use_store::<store::CheckedUsernames>();
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
    html! {
        <div>
            {checkboxes}
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
            post_destination("edited destination", d.value.as_ref().unwrap());
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

fn post_destination(msg: &str, d: &Destination) {
    log!(JsValue::from(msg));
    let d = d.clone();
    yew::platform::spawn_local(async move {
        reqwest::Client::new()
            .post("http://127.0.0.1:3030/api/destinations")
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
            .post("http://127.0.0.1:3030/api/scores")
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
        post_destination("sending new destination to database", &d.value);
        Rc::new(store::NewDestination::default())
    });
    html! {
        <tr>
            <td><input value={new_dest.value.name.to_string()} oninput={oninput_name} /></td>
            <td><textarea value={new_dest.value.description.to_string()} oninput={oninput_desc} /></td>
            <td><button {onclick}>{"create"}</button></td>
            <td>{"unscored"}</td>
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct DestinationCProps {
    pub dest: Destination,
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
    let edit_button = {
        let dest = props.dest.clone();
        let onclick = editing_dest_dispatch.reduce_mut_callback_with(move |d, _| {
            d.value = Some(dest.clone());
        });
        html! {
            <button {onclick}>{"Edit"}</button>
        }
    };
    let score_html = if username.value != store::DEFAULT_USERNAME {
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
    html! {
        <div>
            <div class="content">
                <b>{props.heading.clone()}</b>
            </div>
            <input value={props.text.clone()} oninput={props.oninput.clone()} />
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
