use yew::prelude::*;
use yewdux::prelude::*;

use components::{DestEditC, ScoresC, Username};
use store::fetch_dests_scores;

mod components;
mod store;

#[function_component]
fn App() -> Html {
    let (editing_dest, _editing_dest_dispatch) = use_store::<store::DestBeingEdited>();
    let bottom = if editing_dest.value.is_none() {
        html! {
            <ScoresC />
        }
    } else {
        html! {
            <DestEditC />
        }
    };
    html! {
        <div>
            <Username />
            {bottom}
        </div>
    }
}

fn main() {
    fetch_dests_scores();
    yew::Renderer::<App>::new().render();
}
