use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};

use crate::api::use_api;

#[derive(Debug, Properties, PartialEq)]
pub struct TodoAddProps {
    pub on_add: Callback<(), ()>,
}

#[function_component(TodoAdd)]
pub fn add_todo(props: &TodoAddProps) -> Html {
    let api = use_api();

    let name = use_state(|| String::new());

    let onchange = {
        let name = name.clone();

        move |e: Event| {
            let target: HtmlInputElement = e.target_dyn_into().unwrap();

            name.set(target.value());
        }
    };

    let onsubmit = {
        let api = api.clone();

        let name = name.clone();

        let on_add = props.on_add.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            let api = api.clone();

            let name = name.clone();

            let on_add = on_add.clone();

            spawn_local(async move {
                api.add_todo(&name).await;

                on_add.emit(());
            });
        }
    };

    html!(
        <form { onsubmit }>
            <div class="form-floating mb-3">
                <input type="text" class="form-control" id="name" placeholder="name" value={ (*name).clone() } { onchange } />
                <label for="name">{ "Name" }</label>
            </div>
            <button class="btn btn-primary btn-lg" type="submit">{ "Add" }</button>
        </form>
    )
}
