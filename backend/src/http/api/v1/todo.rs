use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use crate::{
    http::extractors::AuthUser,
    repository::RepositoryError,
    todo::actions::{self, ActionError},
};

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Action: {0}")]
    Action(#[from] ActionError),
    #[error("Validation: {0}")]
    Validation(#[from] validator::ValidationErrors),
}

fn action_into_response(error: ActionError) -> Response {
    match error {
        ActionError::Repository(inner) => repo_into_response(inner),
        ActionError::Forbidden => (StatusCode::FORBIDDEN, error.to_string()).into_response(),
        ActionError::NotFound => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

fn repo_into_response(error: RepositoryError) -> Response {
    tracing::error!("{}", error);

    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
}

fn validation_into_response(error: validator::ValidationErrors) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(error)).into_response()
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::Action(inner) => action_into_response(inner),
            HandlerError::Validation(inner) => validation_into_response(inner),
        }
    }
}

#[derive(Serialize)]
pub struct Paginated<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize, Validate)]
pub struct NewTodo {
    #[validate(length(min = 5))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct RenameTodo {
    #[validate(length(min = 5))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct TodosQuery {
    #[validate(range(min = 1, max = 25))]
    #[serde(default = "TodosQuery::default_limit")]
    pub limit: u64,
    #[validate(range(min = 0))]
    #[serde(default = "TodosQuery::default_offset")]
    pub offset: u64,
}

impl TodosQuery {
    fn default_limit() -> u64 {
        10
    }

    fn default_offset() -> u64 {
        0
    }
}

pub async fn get_todos(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, HandlerError> {
    query.validate()?;

    let count = actions::get_todos_count(&db, &user.id, &query.limit, &query.offset).await?;

    let data = actions::get_todos(&db, &user.id, &query.limit, &query.offset).await?;

    Ok(Json(Paginated { data, count }))
}

pub async fn get_active_todos(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, HandlerError> {
    query.validate()?;

    let count =
        actions::get_todos_count_by_done(&db, &user.id, false, &query.limit, &query.offset).await?;

    let data =
        actions::get_todos_by_done(&db, &user.id, false, &query.limit, &query.offset).await?;

    Ok(Json(Paginated { data, count }))
}

pub async fn get_completed_todos(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, HandlerError> {
    query.validate()?;

    let count =
        actions::get_todos_count_by_done(&db, &user.id, true, &query.limit, &query.offset).await?;

    let data = actions::get_todos_by_done(&db, &user.id, true, &query.limit, &query.offset).await?;

    Ok(Json(Paginated { data, count }))
}

pub async fn create_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Json(payload): Json<NewTodo>,
) -> Result<impl IntoResponse, HandlerError> {
    payload.validate()?;

    let todo = actions::create_todo(&db, &user.id, &payload.name).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn rename_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<RenameTodo>,
) -> Result<impl IntoResponse, HandlerError> {
    payload.validate()?;

    actions::rename_todo(&db, &user.id, &id, &payload.name).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn done_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    actions::done_todo(&db, &user.id, &id).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn revert_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    actions::revert_todo(&db, &user.id, &id).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn delete_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    actions::delete_todo(&db, &user.id, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_router() -> anyhow::Result<Router> {
    Ok(Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos-active", get(get_active_todos))
        .route("/todos-completed", get(get_completed_todos))
        .route(
            "/todos/:id",
            get(get_todo).patch(rename_todo).delete(delete_todo),
        )
        .route("/todos/:id/done", post(done_todo))
        .route("/todos/:id/revert", post(revert_todo)))
}
