use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::header::Header,
    pages::{sign_in::SignIn, sign_up::SignUp, todos::Todos},
    providers::{auth::AuthProvider, guest::GuestProvider},
};

#[function_component(Router)]
pub fn router() -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    )
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/sign-in")]
    SignIn,
    #[at("/sign-up")]
    SignUp,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/")]
    PrivateRoot,
    #[at("/*")]
    Private,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::PrivateRoot | Route::Private => html!(
            <AuthProvider>
                <Header />
                <Switch<PrivateRoute> render={switch_private} />
            </AuthProvider>
        ),
        Route::SignIn => html!(<GuestProvider><SignIn /></GuestProvider>),
        Route::SignUp => html!(<GuestProvider><SignUp /></GuestProvider>),
        Route::NotFound => html!(<Redirect<Route> to={Route::PrivateRoot} />),
    }
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum PrivateRoute {
    #[at("/")]
    Active,
    #[at("/all")]
    All,
    #[at("/completed")]
    Completed,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_private(route: PrivateRoute) -> Html {
    match route {
        PrivateRoute::Active => html!(<Todos is_completed={ Some(false) } />),
        PrivateRoute::All => html!(<Todos is_completed={ None } />),
        PrivateRoute::Completed => html!(<Todos is_completed={ Some(true) } />),
        PrivateRoute::NotFound => html!(<Redirect<PrivateRoute> to={PrivateRoute::Active} />),
    }
}
