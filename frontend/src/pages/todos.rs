use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    api::{
        types::{TodosDeleteQuery, TodosQuery},
        use_api,
    },
    components::{
        modal::Modal, new_todo::NewTodo, pagination::Pagination, spinner::SpinnerMedium,
        todos_table::TodosTable,
    },
    store::{Action, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TodosProps {
    pub is_completed: Option<bool>,
}

#[function_component(Todos)]
pub fn todos(props: &TodosProps) -> Html {
    let api = use_api();

    use_title(
        match &props.is_completed {
            Some(true) => "Completed | ToDos",
            Some(false) => "Active | ToDos",
            None => "All | ToDos",
        }
        .to_string(),
    );

    let is_completed_handle = use_state(|| props.is_completed);

    let pagination_handle = use_state(|| (10, 0));

    let todos_handle = {
        let api = api.clone();

        let is_completed_handle = is_completed_handle.clone();

        let pagination_handle = pagination_handle.clone();

        use_async_with_options(
            async move {
                let query = TodosQuery {
                    is_completed: *is_completed_handle,
                    limit: pagination_handle.0,
                    offset: pagination_handle.1,
                };

                api.todos(query).await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    let delete_completed_handle = {
        let api = api;

        let todos_handle = todos_handle.clone();

        use_async(async move {
            let result = api
                .delete_todos(TodosDeleteQuery {
                    is_completed: Some(true),
                })
                .await;

            match &result {
                Ok(_) => todos_handle.run(),
                Err(err) => Store::dispatch(Action::AlertError(err.to_string())),
            };

            result
        })
    };

    {
        let is_completed_handle = is_completed_handle;

        let pagination_handle = pagination_handle.clone();

        let todos_handle = todos_handle.clone();

        let is_completed = props.is_completed;

        use_effect_with_deps(
            move |_| {
                is_completed_handle.set(is_completed);

                pagination_handle.set((pagination_handle.0, 0));

                todos_handle.run();
            },
            is_completed,
        );
    }

    let set_pagination = {
        let pagination_handle = pagination_handle.clone();

        let todos_handle = todos_handle.clone();

        move |(limit, offset)| {
            pagination_handle.set((limit as usize, offset as usize));

            todos_handle.run();
        }
    };

    let toggle = use_toggle(false, true);

    let open = {
        let toggle = toggle.clone();

        move |_| {
            toggle.toggle();
        }
    };

    let on_add = {
        let todos_handle = todos_handle.clone();

        let toggle = toggle.clone();

        move |()| {
            todos_handle.run();

            toggle.toggle();
        }
    };

    let on_todo_action = {
        let todos_handle = todos_handle.clone();

        move |()| {
            todos_handle.run();
        }
    };

    let delete_completed = {
        let delete_completed_handle = delete_completed_handle;

        move |_| {
            delete_completed_handle.run();
        }
    };

    html!(
        <main class="relative max-w-screen-md mx-auto">
            <div class="flex justify-end w-full py-2">
                <button onclick={ delete_completed } class="btn btn-ghost mr-2">{ "Delete Completed" }</button>
                <button onclick={ open } class="btn btn-primary">{ "Add" }</button>
            </div>
            if *toggle {
                <Modal toggle={ toggle.clone() }>
                    <NewTodo { on_add } />
                </Modal>
            }
            if let Some(data) = &todos_handle.data {
                if data.data.is_empty() {
                    <h2 class="w-full p-10 text-center">{ "Empty" }</h2>
                } else {
                    <div class="w-full grow">
                        <TodosTable
                            todos={ data.data.clone() }
                            on_todo_action={ on_todo_action }
                        />
                    </div>
                    <div class="flex justify-center">
                        <Pagination
                            limit={ pagination_handle.0 as i32 }
                            offset={ pagination_handle.1 as i32 }
                            count={ data.count as i32 }
                            set_pagination={ set_pagination }
                        />
                    </div>
                }
            }
            if todos_handle.loading {
                <SpinnerMedium />
            }
        </main>
    )
}
