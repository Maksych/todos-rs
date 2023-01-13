use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::use_api,
    components::{Modal, TodoAdd, TodoList},
    store::{use_store, Action},
};

#[function_component(Home)]
pub fn home() -> Html {
    let store = use_store();

    let api = use_api();

    let get_todos = {
        move || {
            let store = store.clone();

            spawn_local(async move {
                let todos = api.get_todos().await;

                store.dispatch(Action::SetTodos(todos));
            });
        }
    };

    use_effect(move || {
        get_todos();
    });

    let is_show = use_state(|| false);

    let onclick = {
        let is_show = is_show.clone();

        move |_| {
            is_show.set(true);
        }
    };

    let on_add = {
        let is_show = is_show.clone();

        Callback::from(move |_: ()| {
            is_show.set(false);
        })
    };

    html!(
        <>
            <div class="d-flex justify-content-end">
                <button class="btn btn-primary" { onclick }>{ "Add todo" }</button>
            </div>

            if *is_show {
                <Modal title="Add todo" is_show={is_show}>
                    <TodoAdd { on_add } />
                </Modal>
            }

            <TodoList />
        </>
    )
}
