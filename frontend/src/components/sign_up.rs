use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::use_api,
    router::Route,
    store::{use_store, Action},
};

#[derive(Default, Clone)]
pub struct Form {
    pub username: String,
    pub password: String,
}

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let store = use_store();

    let api = use_api();

    let form = use_state(|| Form::default());

    let onchange_username = {
        let form = form.clone();

        move |e: Event| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();

            form.set(Form {
                username: input.value(),
                password: form.password.clone(),
            });
        }
    };

    let onchange_password = {
        let form = form.clone();

        move |e: Event| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();

            form.set(Form {
                username: form.username.clone(),
                password: input.value(),
            });
        }
    };

    let onsubmit = {
        let store = store.clone();

        let api = api.clone();

        let form = form.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            let store = store.clone();

            let api = api.clone();

            let form = form.clone();

            spawn_local(async move {
                let token = api.sign_up(&form.username, &form.password).await;

                let user = api.profile_with_token(&token.access).await;

                store.dispatch(Action::SignUp { token, user });
            })
        }
    };

    html!(
        <div class="form-sign w-100 m-auto">
            <form { onsubmit }>
                <h1 class="h3 mb-3 fw-normal text-center">{ "Sign up" }</h1>

                <div class="form-floating mb-3">
                    <input
                        type="text"
                        class="form-control"
                        id="username"
                        placeholder="Username"
                        onchange={ onchange_username }
                        value={ form.username.clone() }
                    />

                    <label for="username" class="form-label">{ "Username" }</label>
                </div>

                <div class="form-floating mb-3">
                    <input
                        type="password"
                        class="form-control"
                        id="password"
                        placeholder="Password"
                        onchange={ onchange_password }
                        value={ form.password.clone() }
                    />

                    <label for="password" class="form-label">{ "Password" }</label>
                </div>

                <button type="submit" class="btn btn-primary btn-lg w-100 mb-3">{ "Sign up" }</button>

                <Link<Route> classes="nav-link text-center" to={Route::SignIn}>{ "Sign in" }</Link<Route>>
            </form>
        </div>
    )
}
