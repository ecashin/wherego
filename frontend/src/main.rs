use yew::prelude::*;
use yewdux::prelude::*;

use components::{ScoresC, Username};

mod components;
mod store;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Username />
            <ScoresC />
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
