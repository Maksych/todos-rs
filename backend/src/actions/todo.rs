use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::Todo,
    query as q,
    repository::{Repo, TodoRepo, UserRepo},
};

use super::Error;

pub async fn get_todos_count(
    db: &PgPool,
    user_id: &Uuid,
    limit: &u64,
    offset: &u64,
) -> Result<i64, Error> {
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
) -> Result<Vec<Todo>, Error> {
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
) -> Result<i64, Error> {
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
) -> Result<Vec<Todo>, Error> {
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

pub async fn create_todo(db: &PgPool, user_id: &Uuid, name: &str) -> Result<Todo, Error> {
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

pub async fn get_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, Error> {
    let user = UserRepo::new(db).get_by_id(user_id).await?;

    let todo = TodoRepo::new(db).get_by_id(id).await?;

    if todo.user_id != user.id {
        return Err(Error::Forbidden);
    }

    Ok(todo)
}

pub async fn rename_todo(
    db: &PgPool,
    user_id: &Uuid,
    id: &Uuid,
    name: &str,
) -> Result<Todo, Error> {
    let user = UserRepo::new(db).get_by_id(user_id).await?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?;

    if todo.user_id != user.id {
        return Err(Error::Forbidden);
    }

    todo.name = name.to_owned();

    Ok(repo.update(todo).await?)
}

pub async fn done_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, Error> {
    let user = UserRepo::new(db).get_by_id(user_id).await?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?;

    if todo.user_id != user.id {
        return Err(Error::Forbidden);
    }

    let now = Utc::now();

    todo.is_done = true;
    todo.done_at = Some(now);
    todo.updated_at = now;

    Ok(repo.update(todo).await?)
}

pub async fn revert_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<Todo, Error> {
    let user = UserRepo::new(db).get_by_id(user_id).await?;

    let repo = TodoRepo::new(db);

    let mut todo = repo.get_by_id(id).await?;

    if todo.user_id != user.id {
        return Err(Error::Forbidden);
    }

    todo.is_done = false;
    todo.done_at = None;
    todo.updated_at = Utc::now();

    Ok(repo.update(todo).await?)
}

pub async fn delete_todo(db: &PgPool, user_id: &Uuid, id: &Uuid) -> Result<(), Error> {
    let user = UserRepo::new(db).get_by_id(user_id).await?;

    let repo = TodoRepo::new(db);

    let todo = repo.get_by_id(id).await?;

    if todo.user_id != user.id {
        return Err(Error::Forbidden);
    }

    Ok(repo.delete(todo).await?)
}
