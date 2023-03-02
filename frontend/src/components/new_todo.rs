use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    api::{
        error::FieldError,
        ext::{ApiErrorOptionExt, FieldErrorsMessagesExt},
        types::NewTodo as _NewTodo,
        use_api,
    },
    components::text_input::TextInput,
    store::{Action, Store},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Form {
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Errors {
    pub name: Vec<FieldError>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NewTodoProps {
    pub on_add: Callback<(), ()>,
}

#[function_component(NewTodo)]
pub fn new_todo(props: &NewTodoProps) -> Html {
    let api = use_api();

    let form_handle = use_state(Form::default);

    let add_todo_handle = {
        let form_handle = form_handle.clone();

        let on_add = props.on_add.clone();

        use_async(async move {
            let new_todo = _NewTodo {
                name: form_handle.name.clone(),
            };

            let result = api.new_todo(new_todo).await;

            match &result {
                Ok(_) => on_add.emit(()),
                Err(err) => Store::dispatch(Action::AlertError(err.to_string())),
            };

            result
        })
    };

    let errors = add_todo_handle.error.json::<Errors>();

    let set_name = {
        let form_handle = form_handle.clone();

        move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();

            form_handle.set(Form {
                name: input.value(),
            });
        }
    };

    let submit = {
        let add_todo_handle = add_todo_handle.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            add_todo_handle.run();
        }
    };

    html!(
        <form class="space-y-6 p-3" onsubmit={ submit }>
            <div>
                <h2 class="text-xl font-bold tracking-tight">
                    { "Add todo" }
                </h2>
            </div>

            <div class="-space-y-px rounded-md shadow-sm">
                <div class="mb-3">
                    <TextInput
                        id="name"
                        name="name"
                        r#type="text"
                        onchange={ set_name }
                        placeholder="Name"
                        required={ true }
                        value={ form_handle.name.clone() }
                        errors={ errors.name.clone().messages() }
                    />
                </div>
            </div>

            <div>
                if add_todo_handle.loading {
                    <button class="btn loading"></button>
                } else {
                    <button class="btn btn-primary">
                        { "Add" }
                    </button>
                }
            </div>
        </form>
    )
}
