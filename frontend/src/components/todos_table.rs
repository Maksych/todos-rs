use yew::prelude::*;

use crate::{api::types::Todo, components::todo_row::TodoRow};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TodosTableProps {
    pub todos: Vec<Todo>,
    pub on_todo_action: Callback<(), ()>,
}

#[function_component(TodosTable)]
pub fn todos_table(props: &TodosTableProps) -> Html {
    html!(
        <div class="w-full h-full  overflow-y-auto">
            <table class="table w-full">
                <thead>
                    <tr>
                        <th></th>
                        <th>{ "Name" }</th>
                        <th>{ "Created at" }</th>
                        <th>{ "Updated at" }</th>
                        <th>{ "Completed at" }</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {
                        props
                            .todos
                            .iter()
                            .map(|todo| html!(
                                <TodoRow
                                    todo={ todo.clone() }
                                    on_todo_action={ props.on_todo_action.clone() }
                                />
                            ))
                            .collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    )
}
