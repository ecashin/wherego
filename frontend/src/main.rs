use yew::prelude::*;

use components::{ScoresC, Username};
use store::fetch_dests_scores;

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
    fetch_dests_scores();
    yew::Renderer::<App>::new().render();
}
