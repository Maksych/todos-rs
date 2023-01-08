use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    router::PrivateRoute,
    store::{use_store, Action},
};

#[derive(Properties, PartialEq)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub to: R,
    pub children: Children,
}

#[function_component(CustomLink)]
fn custom_link<R>(props: &Props<R>) -> Html
where
    R: Routable + 'static,
{
    let route = use_route::<R>().unwrap();

    let classes = classes!("nav-link", if route == props.to { "active" } else { "" });

    html!(
        <Link<R>
            classes={classes}
            to={props.to.clone()}>
            {for props.children.iter() }
        </Link<R>>
    )
}

#[function_component(Header)]
pub fn header() -> Html {
    let store = use_store();

    let onclick = {
        let store = store.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            store.dispatch(Action::SignOut);
        }
    };

    html!(
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container">
                <Link<PrivateRoute>
                    classes="navbar-brand"
                    to={PrivateRoute::Home}>
                    { "Todos" }
                </Link<PrivateRoute>>
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                    <li class="nav-item">
                        <CustomLink<PrivateRoute>
                            to={PrivateRoute::Home}>
                            { "All" }
                        </CustomLink<PrivateRoute>>
                    </li>

                    <li class="nav-item">
                        <CustomLink<PrivateRoute>
                            to={PrivateRoute::Active}>
                            { "Active" }
                        </CustomLink<PrivateRoute>>
                    </li>

                    <li class="nav-item">
                        <CustomLink<PrivateRoute>
                            to={PrivateRoute::Completed}>
                            { "Completed" }
                        </CustomLink<PrivateRoute>>
                    </li>
                </ul>
                <ul class="navbar-nav mb-2 mb-lg-0">
                    <li class="nav-item">
                        <a class="nav-link">{ (*store).clone().auth.user.unwrap().username }</a>
                    </li>

                    <li class="nav-item">
                        <a class="nav-link" href="#" { onclick }><i class="bi bi-box-arrow-right"></i></a>
                    </li>
                </ul>
            </div>
        </nav>
    )
}
