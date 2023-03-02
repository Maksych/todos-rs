use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::{
        error::FieldError,
        ext::{ApiErrorOptionExt, FieldErrorsMessagesExt},
        types::Credentials,
        use_api,
    },
    components::text_input::TextInput,
    router::Route,
    store::{Action, Store},
};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Form {
    username: String,
    password: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Errors {
    #[serde(default = "Vec::new")]
    pub username: Vec<FieldError>,
    #[serde(default = "Vec::new")]
    pub password: Vec<FieldError>,
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let api = use_api();

    use_title("Sign in | ToDos".to_string());

    let form_handle = use_state(Form::default);

    let sign_in_handle = {
        let form_handle = form_handle.clone();

        use_async(async move {
            let credentials = Credentials {
                username: form_handle.username.clone(),
                password: form_handle.password.clone(),
            };

            let result = api.sign_in(credentials).await;

            match &result {
                Ok(token) => {
                    Store::dispatch(Action::SignIn(token.clone()));
                }
                Err(err) => {
                    Store::dispatch(Action::AlertError(err.to_string()));
                }
            };

            result
        })
    };

    let errors = sign_in_handle.error.json::<Errors>();

    let set_username = {
        let form_handle = form_handle.clone();

        move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();

            form_handle.set(Form {
                username: input.value(),
                password: form_handle.password.clone(),
            });
        }
    };

    let set_password = {
        let form_handle = form_handle.clone();

        move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();

            form_handle.set(Form {
                username: form_handle.username.clone(),
                password: input.value(),
            });
        }
    };

    let submit = {
        let sign_in_handle = sign_in_handle.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            sign_in_handle.run();
        }
    };

    html!(
        <div class="flex h-screen items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div class="relative w-full max-w-md space-y-8">
                <div>
                    <h2 class="text-center text-3xl font-bold tracking-tight">{ "Sign in" }</h2>
                </div>
                <form class="mt-8 space-y-6 p-3" action="#" method="POST" onsubmit={ submit }>
                    <div class="-space-y-px rounded-md shadow-sm">
                        <div class="mb-3">
                            <TextInput
                                id="username"
                                name="username"
                                r#type="text"
                                required={ true }
                                placeholder="Username"
                                value={ form_handle.username.clone() }
                                errors={ errors.username.messages() }
                                onchange={ set_username.clone() }
                            />
                        </div>

                        <div>
                            <TextInput
                                id="password"
                                name="password"
                                r#type="password"
                                required={ true }
                                placeholder="Password"
                                value={ form_handle.password.clone() }
                                errors={ errors.password.messages() }
                                onchange={ set_password.clone() }
                            />
                        </div>
                    </div>

                    <div>
                        if sign_in_handle.loading {
                            <button class="btn w-full loading"></button>
                        } else {
                            <button type="submit" class="btn w-full btn-primary">
                                { "Sign in" }
                            </button>
                        }
                        <p class="mt-2 text-center">
                            <Link<Route> to={ Route::SignUp } classes="link hover:text-primary">
                                { "Sign up" }
                            </Link<Route>>
                        </p>
                    </div>
                </form>
            </div>
        </div>
    )
}
