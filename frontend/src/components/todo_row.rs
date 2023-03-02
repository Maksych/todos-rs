use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use yew::{html, prelude::function_component, Callback, Html, Properties};
use yew_hooks::use_async;

use crate::{
    api::{types::Todo, use_api},
    store::{Action, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TodoRowProps {
    pub todo: Todo,
    pub on_todo_action: Callback<(), ()>,
}

#[function_component(TodoRow)]
pub fn todo_row(props: &TodoRowProps) -> Html {
    let api = use_api();

    let toggle_completed_handle = {
        let api = api.clone();

        let todo = props.todo.clone();

        let on_todo_action = props.on_todo_action.clone();

        use_async(async move {
            let result = if todo.is_completed {
                api.revert_todo(todo.id).await
            } else {
                api.complete_todo(todo.id).await
            };

            match &result {
                Ok(_) => on_todo_action.emit(()),
                Err(err) => Store::dispatch(Action::AlertError(err.to_string())),
            };

            result
        })
    };

    let delete_handle = {
        let api = api;

        let id = props.todo.id;

        let on_todo_action = props.on_todo_action.clone();

        use_async(async move {
            let result = api.delete_todo(id).await;

            match &result {
                Ok(_) => on_todo_action.emit(()),
                Err(err) => Store::dispatch(Action::AlertError(err.to_string())),
            };

            result
        })
    };

    let toggle = {
        let toggle_completed_handle = toggle_completed_handle;

        move |_| {
            toggle_completed_handle.run();
        }
    };

    let delete = {
        let delete_handle = delete_handle;

        move |_| {
            delete_handle.run();
        }
    };

    html!(
        <tr>
            <td>
                <div class="form-control">
                    <label class="label cursor-pointer">
                        <input
                            type="checkbox"
                            checked={ props.todo.is_completed }
                            class="checkbox checked:checkbox-primary checkbox-sm"
                            onchange={ toggle }
                        />
                    </label>
                </div>
            </td>
            <td class="whitespace-pre-wrap">{ props.todo.name.clone() }</td>
            <td>{ props.todo.created_at.humanize() }</td>
            <td>{ props.todo.updated_at.humanize() }</td>
            <td>
                if let Some(completed_at) = props.todo.completed_at {
                    { completed_at.humanize() }
                }
            </td>
            <td>
                <button
                    class="btn btn-ghost btn-sm btn-circle"
                    onclick={ delete }
                >
                    { "✕" }
                </button>
            </td>
        </tr>
    )
}

pub trait DateTimeHumanizeExt {
    fn humanize(&self) -> String;
}

impl DateTimeHumanizeExt for DateTime<Utc> {
    fn humanize(&self) -> String {
        thread_local! {
            static TZ_OFFSET: FixedOffset = FixedOffset::east_opt(Local::now().offset().local_minus_utc()).unwrap();
        };

        let dt = TZ_OFFSET.with(|tz_offset| tz_offset.from_utc_datetime(&self.naive_utc()));

        dt.format("%Y-%m-%d %H:%M").to_string()
    }
}
