use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use crate::{
    http::extractors::AuthUser,
    todo::actions::{self, ActionError},
};

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Action: {0}")]
    Action(#[from] ActionError),
    #[error("Validation: {0}")]
    Validation(#[from] validator::ValidationErrors),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::Action(inner) => action_into_response(inner),
            HandlerError::Validation(inner) => validation_into_response(inner),
        }
    }
}

fn action_into_response(error: ActionError) -> Response {
    match error {
        ActionError::Db(inner) => db_into_response(inner),
        ActionError::Forbidden => (StatusCode::FORBIDDEN, error.to_string()).into_response(),
        ActionError::NotFound => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

fn db_into_response(error: DbErr) -> Response {
    tracing::error!("{}", error);

    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
}

fn validation_into_response(error: validator::ValidationErrors) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(error)).into_response()
}

#[derive(Serialize)]
pub struct Paginated<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
    pub count: u64,
}

#[derive(Deserialize, Validate)]
pub struct NewTodo {
    #[validate(length(min = 5, message = "Too short"))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 5, message = "Too short"))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct TodosQuery {
    pub is_completed: Option<bool>,
    #[validate(range(min = 1, max = 25))]
    #[serde(default = "TodosQuery::default_limit")]
    pub limit: u64,
    #[validate(range(min = 0))]
    #[serde(default = "TodosQuery::default_offset")]
    pub offset: u64,
}

#[derive(Deserialize, Validate)]
pub struct TodosDeleteQuery {
    pub is_completed: Option<bool>,
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
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, HandlerError> {
    query.validate()?;

    let count = actions::get_todos_count(&db, &user.id, query.is_completed).await?;

    let data = actions::get_todos(
        &db,
        &user.id,
        query.is_completed,
        &query.limit,
        &query.offset,
    )
    .await?;

    Ok(Json(Paginated { data, count }))
}

pub async fn delete_todos(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, HandlerError> {
    actions::delete_todos(&db, &user.id, query.is_completed).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Json(payload): Json<NewTodo>,
) -> Result<impl IntoResponse, HandlerError> {
    payload.validate()?;

    let todo = actions::create_todo(&db, &user.id, &payload.name).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn update_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, HandlerError> {
    payload.validate()?;

    Ok(Json(
        actions::update_todo(&db, &user.id, &id, &payload.name).await?,
    ))
}

pub async fn delete_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    actions::delete_todo(&db, &user.id, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn complete_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(actions::complete_todo(&db, &user.id, &id).await?))
}

pub async fn revert_todo(
    Extension(db): Extension<DatabaseConnection>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(actions::revert_todo(&db, &user.id, &id).await?))
}

pub async fn create_router() -> anyhow::Result<Router> {
    Ok(Router::new()
        .route(
            "/todos",
            get(get_todos).post(create_todo).delete(delete_todos),
        )
        .route(
            "/todos/:id",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route("/todos/:id/complete", post(complete_todo))
        .route("/todos/:id/revert", post(revert_todo)))
}
