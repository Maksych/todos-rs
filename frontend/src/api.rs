use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

use self::{
    error::ApiError,
    types::{Credentials, NewTodo, Paginated, RenameTodo, Todo, TodosDeleteQuery, TodosQuery},
};
use crate::store::{Action, Store, Token, User};

pub mod error;
pub mod ext;
pub mod types;

static BASE_URL: &str = env!("BASE_URL");

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Clone)]
pub struct Api {
    pub token: Option<Token>,
}

macro_rules! impl_methods {
    ($(($method:ident, $method_with_auth:ident),)+) => {
        $(
            fn $method(&self, url: &str) -> RequestBuilder {
                Client::new().$method(format!("{BASE_URL}{url}"))
            }

            async fn $method_with_auth(&self, url: &str) -> ApiResult<RequestBuilder> {
                self.try_auth(self.$method(url)).await
            }
        )+
    };
}

impl Api {
    fn new(token: Option<Token>) -> Self {
        Self { token }
    }

    impl_methods!(
        (get, get_with_auth),
        (post, post_with_auth),
        (patch, patch_with_auth),
        (delete, delete_with_auth),
    );

    async fn try_auth(&self, builder: RequestBuilder) -> ApiResult<RequestBuilder> {
        let token = &self
            .token
            .as_ref()
            .ok_or_else(|| ApiError::Unauthorized("Unauthorized".to_string()))?;

        if !token.access_claims.as_ref().unwrap().is_expired() {
            Ok(builder.bearer_auth(&token.access))
        } else if !token.refresh_claims.as_ref().unwrap().is_expired() {
            let token = self.sign_refresh().await?;

            Store::dispatch(Action::SetToken(Some(token.clone())));

            Ok(builder.bearer_auth(&token.access))
        } else {
            Err(ApiError::TokenExpired)
        }
    }

    async fn json<T>(response: Response) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(response.json::<T>().await?),
            StatusCode::BAD_REQUEST => Err(ApiError::BadRequest(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized(response.text().await?)),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden(response.text().await?)),
            StatusCode::UNPROCESSABLE_ENTITY => Err(ApiError::UnprocessableEntity(
                response.json::<Value>().await?,
            )),
            _ => Err(ApiError::Reqwest(response.text().await?)),
        }
    }

    async fn text(response: Response) -> ApiResult<String> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                Ok(response.text().await?)
            }
            StatusCode::BAD_REQUEST => Err(ApiError::BadRequest(response.text().await?)),
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized(response.text().await?)),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden(response.text().await?)),
            StatusCode::UNPROCESSABLE_ENTITY => Err(ApiError::UnprocessableEntity(
                response.json::<Value>().await?,
            )),
            _ => Err(ApiError::Reqwest(response.text().await?)),
        }
    }

    pub async fn profile(&self) -> ApiResult<User> {
        let response = self.get_with_auth("/profile").await?.send().await?;

        Api::json(response).await
    }

    pub async fn sign_in(&self, credentials: Credentials) -> ApiResult<Token> {
        let response = self.post("/sign-in").json(&credentials).send().await?;

        Api::json(response).await
    }

    pub async fn sign_up(&self, credentials: Credentials) -> ApiResult<Token> {
        let response = self.post("/sign-up").json(&credentials).send().await?;

        Api::json(response).await
    }

    pub async fn sign_refresh(&self) -> ApiResult<Token> {
        let response = match &self.token {
            Some(token) => {
                self.post("/sign-refresh")
                    .bearer_auth(&token.refresh)
                    .send()
                    .await?
            }
            None => return Err(ApiError::TokenExpired),
        };

        Api::json(response).await
    }

    pub async fn todos(&self, query: TodosQuery) -> ApiResult<Paginated<Todo>> {
        let response = self
            .get_with_auth("/todos")
            .await?
            .query(&query)
            .send()
            .await?;

        Api::json(response).await
    }

    pub async fn new_todo(&self, todo: NewTodo) -> ApiResult<Todo> {
        let response = self
            .post_with_auth("/todos")
            .await?
            .json(&todo)
            .send()
            .await?;

        Api::json(response).await
    }

    pub async fn delete_todos(&self, query: TodosDeleteQuery) -> ApiResult<String> {
        let response = self
            .delete_with_auth("/todos")
            .await?
            .query(&query)
            .send()
            .await?;

        Api::text(response).await
    }

    pub async fn get_todo(&self, id: Uuid) -> ApiResult<Todo> {
        let response = self
            .get_with_auth(&format!("/todos/{id}"))
            .await?
            .send()
            .await?;

        Api::json(response).await
    }

    pub async fn update_todo(&self, id: Uuid, todo: RenameTodo) -> ApiResult<Todo> {
        let response = self
            .patch_with_auth(&format!("/todos/{id}"))
            .await?
            .json(&todo)
            .send()
            .await?;

        Api::json(response).await
    }

    pub async fn delete_todo(&self, id: Uuid) -> ApiResult<()> {
        self.delete_with_auth(&format!("/todos/{id}"))
            .await?
            .send()
            .await?;

        Ok(())
    }

    pub async fn complete_todo(&self, id: Uuid) -> ApiResult<Todo> {
        let response = self
            .post_with_auth(&format!("/todos/{id}/complete"))
            .await?
            .send()
            .await?;

        Api::json(response).await
    }

    pub async fn revert_todo(&self, id: Uuid) -> ApiResult<Todo> {
        let response = self
            .post_with_auth(&format!("/todos/{id}/revert"))
            .await?
            .send()
            .await?;

        Api::json(response).await
    }
}

#[hook]
pub fn use_api() -> Api {
    let token = use_selector(|store: &Store| store.token.clone());

    Api::new(token.as_ref().clone())
}
