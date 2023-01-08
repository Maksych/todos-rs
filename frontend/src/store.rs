use std::rc::Rc;

use chrono::{DateTime, Utc};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Store {
    pub auth: Auth,
    pub todos: Vec<Todo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Auth {
    pub token: Option<Token>,
    pub user: Option<User>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Token {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done_at: Option<DateTime<Utc>>,
}

pub enum Action {
    SetToken(Token),
    SetUser(User),
    SignUp { token: Token, user: User },
    SignIn { token: Token, user: User },
    SignRefresh { access: String, refresh: String },
    SetTodos(Vec<Todo>),
    SignOut,
}

impl Reducible for Store {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::SetToken(token) => Self {
                auth: Auth {
                    token: Some(token),
                    user: None,
                },
                ..(*self).clone()
            },

            Action::SetUser(user) => Self {
                auth: Auth {
                    token: self.auth.token.clone(),
                    user: Some(user),
                },
                ..(*self).clone()
            },

            Action::SignIn { token, user } => {
                LocalStorage::set("token", token.clone()).unwrap();

                Self {
                    auth: Auth {
                        token: Some(token),
                        user: Some(user),
                    },
                    ..(*self).clone()
                }
            }

            Action::SignUp { token, user } => {
                LocalStorage::set("token", token.clone()).unwrap();

                Self {
                    auth: Auth {
                        token: Some(token),
                        user: Some(user),
                    },
                    ..(*self).clone()
                }
            }

            Action::SignRefresh { access, refresh } => {
                let token = Token { access, refresh };

                LocalStorage::set("token", token.clone()).unwrap();

                Self {
                    auth: Auth {
                        token: Some(token),
                        user: self.auth.user.clone(),
                    },
                    ..(*self).clone()
                }
            }

            Action::SetTodos(todos) => Self {
                todos,
                ..(*self).clone()
            },

            Action::SignOut => {
                LocalStorage::delete("token");

                Self {
                    auth: Auth {
                        token: None,
                        user: None,
                    },
                    ..(*self).clone()
                }
            }
        }
        .into()
    }
}

pub type StoreContext = UseReducerHandle<Store>;

#[hook]
pub fn use_store() -> StoreContext {
    use_context::<StoreContext>().unwrap()
}

#[derive(Properties, PartialEq)]
pub struct StoreProviderProps {
    pub children: Children,
}

#[function_component(StoreProvider)]
pub fn store_provider(props: &StoreProviderProps) -> Html {
    let store = use_reducer(|| {
        let token: Option<Token> = LocalStorage::get("token").ok();

        Store {
            auth: Auth { token, user: None },
            todos: Vec::new(),
        }
    });

    html!(
        <ContextProvider<StoreContext> context={ store }>
            { props.children.clone() }
        </ContextProvider<StoreContext>>
    )
}
