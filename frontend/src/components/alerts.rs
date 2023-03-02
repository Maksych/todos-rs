use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::alert::Alert,
    store::{Action, Store},
};

#[function_component(Alerts)]
pub fn alerts() -> Html {
    let alerts = use_selector(|store: &Store| store.alerts.clone());

    let timeout = use_timeout(|| Store::dispatch(Action::PopAlert), 1_500);

    if !alerts.is_empty() {
        timeout.reset();
    }

    html!(
        <div class="toast toast-start">
            {
                alerts
                    .as_ref()
                    .clone()
                    .iter()
                    .rev()
                    .map(|alert| html!(
                        <Alert alert={ alert.clone() } />
                    ))
                    .collect::<Html>()
            }
        </div>
    )
}
