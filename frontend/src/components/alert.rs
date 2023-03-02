use yew::prelude::*;

use crate::store::{Alert as _Alert, AlertType};

#[derive(Debug, PartialEq, Properties)]
pub struct AlertProps {
    pub alert: _Alert,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let _alert = &props.alert;

    let (class, d) = match _alert.r#type {
        AlertType::Success => (
            "alert-success",
            "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
        ),
        AlertType::Warning => (
            "alert-warning",
            "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
        ),
        AlertType::Error => (
            "alert-error",
            "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
        ),
    };

    html!(
        <div class={classes!("alert", class, "shadow-lg", "m-3")}>
            <div>
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" { d } />
                </svg>

                <span>{ _alert.text.clone() }</span>
            </div>
        </div>
    )
}
