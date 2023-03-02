use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PaginationProps {
    pub limit: i32,
    pub offset: i32,
    pub count: i32,
    pub set_pagination: Callback<(i32, i32), ()>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let limit = props.limit;
    let offset = props.offset;
    let count = props.count;

    let disabled_prev = offset <= 0;

    let disabled_next = offset + limit >= count;

    let set_first = {
        let set_pagination = props.set_pagination.clone();

        move |_| {
            set_pagination.emit((limit, 0));
        }
    };

    let set_prev = {
        let set_pagination = props.set_pagination.clone();

        move |_| {
            set_pagination.emit((limit, offset - limit));
        }
    };

    let set_next = {
        let set_pagination = props.set_pagination.clone();

        move |_| {
            set_pagination.emit((limit, offset + limit));
        }
    };

    let set_last = {
        let set_pagination = props.set_pagination.clone();

        move |_| {
            set_pagination.emit((limit, count / limit * limit));
        }
    };

    let page = offset / limit + 1;

    html!(
        <div class="btn-group">
            <button class="btn" onclick={ set_first } disabled={ disabled_prev }>{ "<<" }</button>
            <button class="btn" onclick={ set_prev } disabled={ disabled_prev }>{ "<" }</button>
            <button class="btn">{ page }</button>
            <button class="btn" onclick={ set_next } disabled={ disabled_next }>{ ">" }</button>
            <button class="btn" onclick={ set_last } disabled={ disabled_next }>{ ">>" }</button>
        </div>
    )
}
