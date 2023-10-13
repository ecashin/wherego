use yew::prelude::*;
use yewdux::prelude::*;

use components::DestinationReadOnly;

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
                <DestinationReadOnly dest={d.clone()} />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <table>
           {dests_html}
        </table>
    }
}

fn main() {
    let dest_dispatch = Dispatch::<store::Destinations>::new();
    yew::platform::spawn_local(async move {
        let sent = reqwest::get("http://127.0.0.1:3030/api/destinations")
            .await
            .unwrap();
        let received = sent.json().await.unwrap();
        dest_dispatch.set(store::Destinations { value: received });
    });

    yew::Renderer::<App>::new().render();
}
