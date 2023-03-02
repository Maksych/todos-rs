use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    router::PrivateRoute,
    store::{Action, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NavLinkProps {
    pub to: PrivateRoute,
    pub children: Children,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let route = use_route::<PrivateRoute>().unwrap();

    let classes = if route == props.to {
        classes!("bg-primary", "text-white")
    } else {
        classes!()
    };

    html!(
        <Link<PrivateRoute> to={ props.to.clone() } classes={ classes }>
            { for props.children.iter() }
        </Link<PrivateRoute>>
    )
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user = use_selector(|store: &Store| store.user.clone());

    let sign_out = |_| Store::dispatch(Action::SignOut);

    html!(
        <div class="navbar rounded-lg bg-base-300">
            <div class="navbar-start">
                <Link<PrivateRoute>
                    to={ PrivateRoute::Active }
                    classes={ classes!("btn", "btn-ghost", "normal-case", "text-xl") }
                >
                    { "Todos" }
                </Link<PrivateRoute>>
            </div>
            <div class="navbar-center">
                <ul class="menu menu-horizontal px-1 space-x-4">
                    <li>
                        <NavLink to={ PrivateRoute::Active }>{ "Active" }</NavLink>
                    </li>
                    <li>
                        <NavLink to={ PrivateRoute::All }>{ "All" }</NavLink>
                    </li>
                    <li>
                        <NavLink to={ PrivateRoute::Completed }>{ "Completed" }</NavLink>
                    </li>
                </ul>
            </div>
            <div class="navbar-end">
                <button onclick={ sign_out } class="btn btn-ghost gap-2">
                    if let Some(user) = user.as_ref() {
                        { user.username.clone() }
                    }
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                    </svg>
                </button>
            </div>
        </div>
    )
}
