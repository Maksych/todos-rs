use yew::prelude::*;

use crate::{components::alerts::Alerts, router::Router};

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <>
            <Router />
            <Alerts />
        </>
    )
}
