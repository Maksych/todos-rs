use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::use_api,
    components::{Active, Completed, Header, Home, SignIn, SignUp},
    store::{use_store, Action},
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    PrivateRoot,
    #[at("/*")]
    Private,
    #[at("/sign-in")]
    SignIn,
    #[at("/sign-up")]
    SignUp,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PrivateRoute {
    #[at("/")]
    Home,
    #[at("/active")]
    Active,
    #[at("/completed")]
    Completed,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::PrivateRoot | Route::Private => html!(
            <AuthProvider>
                <Header />
                <div class="container">
                    <Switch<PrivateRoute> render={ switch_private } />
                </div>
            </AuthProvider>
        ),
        Route::SignIn => html!( <GuestProvider><SignIn /></GuestProvider> ),
        Route::SignUp => html!( <GuestProvider><SignUp /></GuestProvider> ),
        Route::NotFound => html!( <Redirect<Route> to={ Route::PrivateRoot } /> ),
    }
}

pub fn switch_private(route: PrivateRoute) -> Html {
    match route {
        PrivateRoute::Home => html!( <Home /> ),
        PrivateRoute::Active => html!( <Active /> ),
        PrivateRoute::Completed => html!( <Completed /> ),
        PrivateRoute::NotFound => html!( <Redirect<Route> to={ Route::PrivateRoot } /> ),
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct GuestProviderProps {
    pub children: Children,
}

#[function_component(GuestProvider)]
pub fn guest_provider(props: &GuestProviderProps) -> Html {
    let store = use_store();

    if store.auth.token.is_some() {
        html!(
            <Redirect<Route> to={ Route::PrivateRoot } />
        )
    } else {
        html!(
            <>{ for props.children.iter() }</>
        )
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let store = use_store();

    let api = use_api();

    if store.auth.token.is_none() {
        html!(
            <Redirect<Route> to={ Route::SignIn } />
        )
    } else if store.auth.token.is_some() && store.auth.user.is_none() {
        spawn_local(async move {
            let user = api.profile().await;

            store.dispatch(Action::SetUser(user));
        });

        html!(
            <div class="spinner-container d-flex justify-content-center align-items-center">
                <div class="spinner-border">
                    <span class="visually-hidden">{ "Loading..." }</span>
                </div>
            </div>
        )
    } else {
        html!(
            <>{ for props.children.iter() }</>
        )
    }
}
