use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    api::use_api,
    components::spinner::SpinnerLarge,
    router::Route,
    store::{Action, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let api = use_api();

    let token = use_selector(|store: &Store| store.token.clone());

    let user = use_selector(|store: &Store| store.user.clone());

    use_async_with_options(
        async move {
            match api.profile().await {
                Ok(user) => Store::dispatch(Action::SetUser(Some(user))),
                Err(err) => Store::dispatch(Action::SignReject(err.to_string())),
            };

            Ok(()) as Result<(), ()>
        },
        UseAsyncOptions::enable_auto(),
    );

    match (token.as_ref(), user.as_ref()) {
        (Some(_), Some(_)) => html!(<>{ for props.children.iter() }</>),
        (Some(_), None) => html!(<SpinnerLarge />),
        _ => html!(<Redirect<Route> to={Route::SignIn} />),
    }
}
