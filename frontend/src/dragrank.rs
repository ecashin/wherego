use gloo_console::log;
use stylist::yew::styled_component;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yewdux::prelude::*;

use wherego::Destination;

use crate::store;

#[function_component]
pub fn DestinationList() -> Html {
    let (dests, dests_dispatch) = use_store::<store::Destinations>();
    let list_items = dests
        .value
        .iter()
        .map(|d| {
            html! {
                <DestListItem dest={d.clone()} />
            }
        })
        .collect::<Vec<_>>();
    html! {
        <ul>
            {list_items}
        </ul>
    }
}

#[derive(PartialEq, Properties)]
pub struct DestListItemProps {
    pub dest: Destination,
}

#[styled_component]
pub fn DestListItem(props: &DestListItemProps) -> Html {
    let dest_name = props.dest.name.clone();
    let dest_id = props.dest.id;
    let (selected_id, select_dispatch) = use_store::<store::SelectedDestinationId>();
    let (over_id, over_dispatch) = use_store::<store::OverDestinationId>();
    let (dests, dests_dispatch) = use_store::<store::Destinations>();
    let ondragstart = {
        let dest_name = dest_name.clone();
        select_dispatch.reduce_mut_callback_with(move |s, _e| {
            log!(JsValue::from(&format!(
                "dragging destination with id {dest_id}: {}",
                dest_name
            )));
            s.value = dest_id;
        })
    };
    let ondragover = over_dispatch.reduce_mut_callback_with(move |over, _e| {
        if selected_id.value != -1 {
            let msg = format!("dragging {} over {}", selected_id.value, dest_id);
            log!(JsValue::from(&msg));
            over.value = dest_id;
        }
    });
    let ondragend = {
        dests_dispatch.reduce_mut_callback(move |dests| {
            let selected_dispatch = Dispatch::<store::SelectedDestinationId>::new();
            let selected_id = selected_dispatch.get().value;
            let over_id = Dispatch::<store::OverDestinationId>::new().get().value;
            let new_order = move_before(&dests.value, selected_id, over_id);
            let label = |d: &Destination| format!("{}:({})", d.id, d.name);
            let msg = format!(
                "dests: {:?} => {:?}",
                dests.value.iter().map(|d| label(d)).collect::<Vec<_>>(),
                new_order.iter().map(|d| label(d)).collect::<Vec<_>>(),
            );
            log!(JsValue::from(&msg));
            dests.value = new_order;
            selected_dispatch.set(store::SelectedDestinationId { value: -1 });
        })
    };
    html! {
        <li
            class={css!("cursor: pointer; user-select: none;")}
            draggable={"true"}
            {ondragstart}
            {ondragover}
            {ondragend}
        >{&dest_name}</li>
    }
}

fn move_before(dests: &Vec<Destination>, selected_id: i64, over_id: i64) -> Vec<Destination> {
    if selected_id == -1 || over_id == -1 {
        log!(JsValue::from("useless copy"));
        return dests.clone();
    }
    if selected_id == over_id {
        log!(JsValue::from("not moving over self copy"));
        return dests.clone();
    }
    let (selected_dest, selected_index) = dests
        .iter()
        .enumerate()
        .find_map(|(i, d)| {
            if d.id == selected_id {
                Some((d.clone(), i))
            } else {
                None
            }
        })
        .unwrap();
    let msg = format!(
        "removing {} at index {}",
        &selected_dest.name, selected_index
    );
    log!(JsValue::from(&msg));
    let mut new_order = dests.clone();
    new_order.remove(selected_index);
    let over_index = new_order
        .iter()
        .enumerate()
        .find_map(|(i, d)| if d.id == over_id { Some(i) } else { None })
        .unwrap();
    new_order.insert(over_index, selected_dest);
    new_order
}
