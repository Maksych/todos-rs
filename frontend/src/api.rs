use gloo_net::http::{Method, Request};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use yew::prelude::*;

use crate::store::{use_store, StoreContext, Todo, Token, User};

#[derive(Debug, Clone)]
pub struct Api {
    pub base_url: String,
    pub store: StoreContext,
}

impl Api {
    pub fn new(base_url: String, store: StoreContext) -> Self {
        Self { base_url, store }
    }

    pub fn request(&self, url: &str) -> Request {
        let url = self.base_url.clone() + url;

        Request::new(&url)
    }

    pub fn request_with_auth(&self, url: &str) -> Request {
        let token = (*self.store).clone().auth.token.unwrap().access;

        self.request_with_token(url, &token)
    }

    pub fn request_with_token(&self, url: &str, token: &str) -> Request {
        self.request(url)
            .header("Authorization", &format!("Bearer {}", token))
    }

    pub async fn sign_up(&self, username: &str, password: &str) -> Token {
        self.request("sign-up")
            .method(Method::POST)
            .json(&json!({"username": username, "password": password}))
            .unwrap()
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn sign_in(&self, username: &str, password: &str) -> Token {
        self.request("sign-in")
            .method(Method::POST)
            .json(&json!({"username": username, "password": password}))
            .unwrap()
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn profile(&self) -> User {
        self.request_with_auth("profile")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn profile_with_token(&self, token: &str) -> User {
        self.request_with_token("profile", token)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn get_todos(&self) -> Vec<Todo> {
        self.request_with_auth("todos")
            .send()
            .await
            .unwrap()
            .json::<Paginated<Todo>>()
            .await
            .unwrap()
            .data
    }

    pub async fn get_active_todos(&self) -> Vec<Todo> {
        self.request_with_auth("todos-active")
            .send()
            .await
            .unwrap()
            .json::<Paginated<Todo>>()
            .await
            .unwrap()
            .data
    }

    pub async fn get_completed_todos(&self) -> Vec<Todo> {
        self.request_with_auth("todos-completed")
            .send()
            .await
            .unwrap()
            .json::<Paginated<Todo>>()
            .await
            .unwrap()
            .data
    }

    pub async fn add_todo(&self, name: &str) -> Todo {
        self.request_with_auth("todos")
            .method(Method::POST)
            .json(&json!({ "name": name }))
            .unwrap()
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn done_todo(&self, id: Uuid) -> Todo {
        self.request_with_auth(&format!("todos/{}/done", id))
            .method(Method::POST)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn revert_todo(&self, id: Uuid) -> Todo {
        self.request_with_auth(&format!("todos/{}/revert", id))
            .method(Method::POST)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn remove_todo(&self, id: Uuid) {
        self.request_with_auth(&format!("todos/{}", id))
            .method(Method::DELETE)
            .send()
            .await
            .unwrap();
    }
}

#[hook]
pub fn use_api() -> Api {
    let store = use_store();

    Api::new("http://127.0.0.1:8080/api/v1/".to_owned(), store)
}

#[derive(Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub count: usize,
}
