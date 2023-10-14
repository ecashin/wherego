use gloo_console::log;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

use wherego::{Destination, Score};

use crate::store;

#[derive(PartialEq, Properties)]
pub struct DestinationCProps {
    pub dest: Destination,
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
    });
}

#[function_component]
pub fn DestinationC(props: &DestinationCProps) -> Html {
    let (username, _) = use_store::<store::Username>();
    let (scores, scores_dispatch) = use_store::<store::Scores>();
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
            <td>{props.dest.id}</td>
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
