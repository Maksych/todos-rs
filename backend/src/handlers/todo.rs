use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    actions,
    models::{AuthUser, NewTodo, Paginated, RenameTodo, TodosQuery},
};

use super::Error;

pub async fn get_todos(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, Error> {
    query.validate()?;

    let count = actions::get_todos_count(&db, &user.id, &query.limit, &query.offset).await?;

    let data = actions::get_todos(&db, &user.id, &query.limit, &query.offset).await?;

    Ok(Json(Paginated { data, count }))
}

pub async fn get_active_todos(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Query(query): Query<TodosQuery>,
) -> Result<impl IntoResponse, Error> {
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
) -> Result<impl IntoResponse, Error> {
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
) -> Result<impl IntoResponse, Error> {
    payload.validate()?;

    Ok(Json(
        actions::create_todo(&db, &user.id, &payload.name).await?,
    ))
}

pub async fn get_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn rename_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<RenameTodo>,
) -> Result<impl IntoResponse, Error> {
    payload.validate()?;

    actions::rename_todo(&db, &user.id, &id, &payload.name).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn done_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    actions::done_todo(&db, &user.id, &id).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn revert_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    actions::revert_todo(&db, &user.id, &id).await?;

    Ok(Json(actions::get_todo(&db, &user.id, &id).await?))
}

pub async fn delete_todo(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    actions::delete_todo(&db, &user.id, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}
