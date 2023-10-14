use yew::prelude::*;
use yewdux::prelude::*;

use components::{DestinationC, Username};

mod components;
mod store;

#[function_component]
fn App() -> Html {
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
        <div>
            <Username />
            <table>
               {dests_html}
            </table>
        </div>
    }
}

fn main() {
    let dest_dispatch = Dispatch::<store::Destinations>::new();
    let scores_dispatch = Dispatch::<store::Scores>::new();
    yew::platform::spawn_local(async move {
        let sent = reqwest::get("http://127.0.0.1:3030/api/destinations")
            .await
            .unwrap();
        let received = sent.json().await.unwrap();
        dest_dispatch.set(store::Destinations { value: received });
        let sent = reqwest::get("http://127.0.0.1:3030/api/scores")
            .await
            .unwrap();
        let received = sent.json().await.unwrap();
        scores_dispatch.set(store::Scores { value: received });
    });

    yew::Renderer::<App>::new().render();
}
