use std::collections::VecDeque;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, TimeZone, Utc};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Store)]
pub struct Store {
    pub alerts: VecDeque<Alert>,
    pub token: Option<Token>,
    pub user: Option<User>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            alerts: VecDeque::new(),
            token: LocalStorage::get::<Token>("token")
                .ok()
                .map(|token| token.with_claims()),
            user: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub r#type: AlertType,
    pub text: String,
}

impl Alert {
    pub fn new(r#type: AlertType, text: &str) -> Self {
        Self {
            r#type,
            text: text.to_string(),
        }
    }

    pub fn new_error(text: &str) -> Self {
        Self::new(AlertType::Error, text)
    }

    pub fn new_warning(text: &str) -> Self {
        Self::new(AlertType::Warning, text)
    }

    pub fn new_success(text: &str) -> Self {
        Self::new(AlertType::Success, text)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    Error,
    Warning,
    Success,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub access: String,
    pub access_claims: Option<Claims>,
    pub refresh: String,
    pub refresh_claims: Option<Claims>,
}

impl Token {
    pub fn with_claims(self) -> Self {
        Self {
            access_claims: Some(Claims::from_token(self.access.clone())),
            refresh_claims: Some(Claims::from_token(self.refresh.clone())),
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub sub: Uuid,
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        Utc.timestamp_opt(self.exp, 0).unwrap() < Utc::now()
    }
}

impl Claims {
    fn from_token(token: String) -> Self {
        let decoded = URL_SAFE_NO_PAD
            .decode(token.split('.').nth(1).unwrap())
            .unwrap();

        serde_json::from_slice(&decoded).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub joined_at: DateTime<Utc>,
}

pub enum Action {
    SignIn(Token),
    SignUp(Token),
    SignOut,
    SignReject(String),
    SetToken(Option<Token>),
    SetUser(Option<User>),
    Alert(Alert),
    AlertSuccess(String),
    AlertWarning(String),
    AlertError(String),
    PopAlert,
}

impl Store {
    pub fn dispatch(action: Action) {
        thread_local! {
            static DISPATCH: Dispatch<Store> = Dispatch::<Store>::new();
        };

        DISPATCH.with(|dispatch| {
            dispatch.reduce_mut(|store| {
                match action {
                    Action::SignIn(token) => Store::sign_in(store, token),
                    Action::SignUp(token) => Store::sign_up(store, token),
                    Action::SignOut => Store::sign_out(store),
                    Action::SignReject(text) => Store::sign_reject(store, text),
                    Action::SetToken(token) => Store::set_token(store, token),
                    Action::SetUser(user) => Store::set_user(store, user),
                    Action::Alert(alert) => Store::alert(store, alert),
                    Action::AlertSuccess(text) => Store::alert(store, Alert::new_success(&text)),
                    Action::AlertWarning(text) => Store::alert(store, Alert::new_warning(&text)),
                    Action::AlertError(text) => Store::alert(store, Alert::new_error(&text)),
                    Action::PopAlert => Store::pop_alert(store),
                };
            });
        });
    }

    fn sign_in(store: &mut Store, token: Token) {
        LocalStorage::set("token", &token).unwrap();

        store.token = Some(token.with_claims());

        store.user = None;
    }

    fn sign_up(store: &mut Store, token: Token) {
        LocalStorage::set("token", &token).unwrap();

        store.token = Some(token.with_claims());

        store.user = None;
    }

    fn sign_out(store: &mut Store) {
        LocalStorage::delete("token");

        store.token = None;

        store.user = None;
    }

    fn sign_reject(store: &mut Store, text: String) {
        LocalStorage::delete("token");

        store.token = None;

        store.user = None;

        store.alerts.push_back(Alert::new_error(&text));
    }

    fn set_token(store: &mut Store, token: Option<Token>) {
        store.token = match token {
            Some(token) => {
                LocalStorage::set("token", &token).unwrap();

                Some(token.with_claims())
            }
            None => {
                LocalStorage::delete("token");

                store.user = None;

                None
            }
        };
    }

    fn set_user(store: &mut Store, user: Option<User>) {
        store.user = user;
    }

    fn alert(store: &mut Store, alert: Alert) {
        store.alerts.push_back(alert);
    }

    fn pop_alert(store: &mut Store) {
        store.alerts.pop_front();
    }
}
