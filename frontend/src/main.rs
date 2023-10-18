use yew::prelude::*;
use yewdux::prelude::*;

use components::{DestEditC, NegotiationResultsC, ScoresC, UserSelectC, Username};
use store::fetch_dests_scores;

mod components;
mod dragrank;
mod store;

pub fn full_url(relative_url: &str) -> String {
    let base_url = &Dispatch::<store::BaseUrl>::new().get().value;
    format!("{base_url}{relative_url}")
}

#[function_component]
fn App() -> Html {
    let (editing_dest, _editing_dest_dispatch) = use_store::<store::DestBeingEdited>();
    let (negotiation_results, _) = use_store::<store::NegotiationResults>();
    let bottom = if editing_dest.value.is_some() {
        html! {
            <>
                <UserSelectC />
                <DestEditC />
            </>
        }
    } else if negotiation_results.value.is_some() {
        html! {
            <NegotiationResultsC />
        }
    } else {
        html! {
            <>
                <UserSelectC />
                <ScoresC />
            </>
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
