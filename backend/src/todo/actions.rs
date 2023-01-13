use chrono::Utc;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    auth::repository::UserRepo,
    repository::{Repository, RepositoryError},
};

use super::{models::Todo, query as q, repository::TodoRepo};

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Repo: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
}

pub async fn get_todos_count(
    db: &PgPool,
    user_id: &Uuid,
    limit: &u64,
    offset: &u64,
) -> Result<i64, ActionError> {
    Ok(TodoRepo::new(db)
        .count(|stmt| {
            stmt.and_where(q::Expr::col(q::Todo::UserId).eq(*user_id))
                .limit(*limit)
                .offset(*offset);
        })
        .await?)
}

pub async fn get_todos(
    db: &PgPool,
    user_id: &Uuid,
    limit: &u64,
    offset: &u64,
) -> Result<Vec<Todo>, ActionError> {
    Ok(TodoRepo::new(db)
        .select(|stmt| {
            stmt.and_where(q::Expr::col(q::Todo::UserId).eq(*user_id))
                .order_by(q::Todo::CreatedAt, q::Order::Desc)
                .limit(*limit)
                .offset(*offset);
        })
        .await?)
}

pub async fn get_todos_count_by_done(
    db: &PgPool,
    user_id: &Uuid,
    is_done: bool,
    limit: &u64,
    offset: &u64,
) -> Result<i64, ActionError> {
    Ok(TodoRepo::new(db)
        .count(|stmt| {
            stmt.and_where(q::Expr::col(q::Todo::UserId).eq(*user_id))
                .and_where(q::Expr::col(q::Todo::IsDone).eq(is_done))
                .limit(*limit)
                .offset(*offset);
        })
        .await?)
}

pub async fn get_todos_by_done(
    db: &PgPool,
    user_id: &Uuid,
    is_done: bool,
    limit: &u64,
    offset: &u64,
) -> Result<Vec<Todo>, ActionError> {
    Ok(TodoRepo::new(db)
        .select(|stmt| {
            stmt.and_where(q::Expr::col(q::Todo::UserId).eq(*user_id))
                .and_where(q::Expr::col(q::Todo::IsDone).eq(is_done))
                .order_by(q::Todo::CreatedAt, q::Order::Desc)
                .limit(*limit)
                .offset(*offset);
        })
        .await?)
}

pub async fn create_todo(db: &PgPool, user_id: &Uuid, name: &str) -> Result<Todo, ActionError> {
    UserRepo::new(db).get_by_id(user_id).await?;

    let now = Utc::now();

    let todo = Todo {
        id: Uuid::new_v4(),
        user_id: user_id.to_owned(),
        name: name.into(),
        is_done: false,
        created_at: now,
        updated_at: now,
        done_at: None,
    };

    Ok(TodoRepo::new(db).insert(todo).await?)
}

pub async fn get_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, ActionError> {
    let user = UserRepo::new(db)
        .get_by_id(user_id)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = TodoRepo::new(db)
        .get_by_id(id)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    Ok(todo)
}

pub async fn rename_todo(
    db: &PgPool,
    user_id: &Uuid,
    id: &Uuid,
    name: &str,
) -> Result<Todo, ActionError> {
    let user = UserRepo::new(db)
        .get_by_id(user_id)
        .await?
        .ok_or(ActionError::NotFound)?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?.ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    todo.name = name.to_owned();

    Ok(repo.update(todo).await?)
}

pub async fn done_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, ActionError> {
    let user = UserRepo::new(db)
        .get_by_id(user_id)
        .await?
        .ok_or(ActionError::NotFound)?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?.ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    let now = Utc::now();

    todo.is_done = true;
    todo.done_at = Some(now);
    todo.updated_at = now;

    Ok(repo.update(todo).await?)
}

pub async fn revert_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, ActionError> {
    let user = UserRepo::new(db)
        .get_by_id(user_id)
        .await?
        .ok_or(ActionError::NotFound)?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?.ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    todo.is_done = false;
    todo.done_at = None;
    todo.updated_at = Utc::now();

    Ok(repo.update(todo).await?)
}

pub async fn delete_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<(), ActionError> {
    let user = UserRepo::new(db)
        .get_by_id(user_id)
        .await?
        .ok_or(ActionError::NotFound)?;

    let repo = TodoRepo::new(db);

    let todo = repo.get_by_id(id).await?.ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    Ok(repo.delete(todo).await?)
}
