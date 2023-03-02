use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{router::Route, store::Store};

#[derive(Debug, PartialEq, Properties)]
pub struct GuestProviderProps {
    pub children: Children,
}

#[function_component(GuestProvider)]
pub fn guest_provider(props: &GuestProviderProps) -> Html {
    let token = use_selector(|store: &Store| store.token.clone());

    match token.as_ref() {
        Some(_) => html!(<Redirect<Route> to={Route::PrivateRoot} />),
        None => html!(<>{ for props.children.iter() }</>),
    }
}
