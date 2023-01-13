use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::use_api,
    store::{use_store, Action},
};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct DoneToggleProps {
    pub id: Uuid,
    pub is_done: bool,
}

#[function_component(DoneToggle)]
pub fn done_toggle(props: &DoneToggleProps) -> Html {
    let store = use_store();

    let api = use_api();

    let onchange = {
        let props = props.clone();

        move |_| {
            let store = store.clone();

            let api = api.clone();

            let props = props.clone();

            spawn_local(async move {
                if props.is_done {
                    api.revert_todo(props.id).await;
                } else {
                    api.done_todo(props.id).await;
                }

                store.dispatch(Action::SetTodos(Vec::new()));
            });
        }
    };

    html!(
        <input
            class="form-check-input"
            type="checkbox"
            id={ props.id.to_string() }
            checked={ props.is_done }
            { onchange } />
    )
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct RemoveProps {
    pub id: Uuid,
}

#[function_component(Remove)]
pub fn remove(props: &RemoveProps) -> Html {
    let store = use_store();

    let api = use_api();

    let onclick = {
        let props = props.clone();

        move |_| {
            let store = store.clone();

            let api = api.clone();

            let props = props.clone();

            spawn_local(async move {
                api.remove_todo(props.id).await;

                store.dispatch(Action::SetTodos(Vec::new()));
            });
        }
    };

    html!(
        <button type="button" class="btn-close" { onclick }></button>
    )
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let store = use_store();

    html!(
        <table class="table table-striped">
            <thead>
                <tr>
                <th scope="col">{ "#" }</th>
                <th scope="col">{ "Done" }</th>
                <th scope="col">{ "Name" }</th>
                <th scope="col">{ "Created at" }</th>
                <th scope="col">{ "Updated at" }</th>
                <th scope="col">{ "Done at" }</th>
                <th scope="col"></th>
                </tr>
            </thead>
            <tbody>
                {
                    store
                        .todos
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, elem)| {
                            html!(
                                <tr key={ elem.id.to_string() }>
                                    <th scope="row">{ idx + 1 }</th>
                                    <td>
                                        <DoneToggle
                                            id={ elem.id }
                                            is_done={ elem.is_done }
                                        />
                                    </td>
                                    <td>{ elem.name }</td>
                                    <td>{ elem.created_at }</td>
                                    <td>{ elem.updated_at }</td>
                                    <td>{ elem.done_at }</td>
                                    <td><Remove id={ elem.id } /></td>
                                </tr>
                            )
                        })
                        .collect::<Html>()
                }
            </tbody>
        </table>
    )
}
