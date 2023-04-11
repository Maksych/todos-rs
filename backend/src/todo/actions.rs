use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entities::{todo, user};

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("DbErr: {0}")]
    Db(#[from] DbErr),
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
}

pub async fn get_todos_count(
    db: &DatabaseConnection,
    user_id: &Uuid,
    is_completed: Option<bool>,
) -> Result<u64, ActionError> {
    let mut stmt = todo::Entity::find().filter(todo::Column::UserId.eq(*user_id));

    if let Some(is_completed) = is_completed {
        stmt = stmt.filter(todo::Column::IsCompleted.eq(is_completed));
    }

    Ok(stmt.count(db).await?)
}

pub async fn get_todos(
    db: &DatabaseConnection,
    user_id: &Uuid,
    is_completed: Option<bool>,
    limit: &u64,
    offset: &u64,
) -> Result<Vec<todo::Model>, ActionError> {
    let mut stmt = todo::Entity::find().filter(todo::Column::UserId.eq(*user_id));

    if let Some(is_completed) = is_completed {
        stmt = stmt.filter(todo::Column::IsCompleted.eq(is_completed));
    }

    Ok(stmt
        .order_by(todo::Column::CreatedAt, Order::Desc)
        .limit(*limit)
        .offset(*offset)
        .all(db)
        .await?)
}

pub async fn create_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    name: &str,
) -> Result<todo::Model, ActionError> {
    user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let now = Utc::now();

    let new_todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id.to_owned()),
        name: Set(name.into()),
        is_completed: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        completed_at: Set(None),
    };

    Ok(new_todo.insert(db).await?)
}

pub async fn delete_todos(
    db: &DatabaseConnection,
    user_id: &Uuid,
    is_completed: Option<bool>,
) -> Result<(), ActionError> {
    let mut stmt = todo::Entity::delete_many().filter(todo::Column::UserId.eq(*user_id));

    if let Some(is_completed) = is_completed {
        stmt = stmt.filter(todo::Column::IsCompleted.eq(is_completed));
    }

    stmt.exec(db).await?;

    Ok(())
}

pub async fn get_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<todo::Model, ActionError> {
    let user = user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = todo::Entity::find_by_id(*id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    Ok(todo)
}

pub async fn update_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    id: &Uuid,
    name: &str,
) -> Result<todo::Model, ActionError> {
    let user = user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = todo::Entity::find_by_id(*id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    let mut todo = todo.into_active_model();

    todo.name = Set(name.to_owned());
    todo.updated_at = Set(Utc::now());

    Ok(todo.update(db).await?)
}

pub async fn delete_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<(), ActionError> {
    let user = user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = todo::Entity::find_by_id(*id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    todo.delete(db).await?;

    Ok(())
}

pub async fn complete_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<todo::Model, ActionError> {
    let user = user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = todo::Entity::find_by_id(*id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    let mut todo = todo.into_active_model();

    let now = Utc::now();

    todo.is_completed = Set(true);
    todo.completed_at = Set(Some(now));
    todo.updated_at = Set(now);

    Ok(todo.update(db).await?)
}

pub async fn revert_todo(
    db: &DatabaseConnection,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<todo::Model, ActionError> {
    let user = user::Entity::find_by_id(*user_id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    let todo = todo::Entity::find_by_id(*id)
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if todo.user_id != user.id {
        return Err(ActionError::Forbidden);
    }

    let mut todo = todo.into_active_model();

    todo.is_completed = Set(false);
    todo.completed_at = Set(None);
    todo.updated_at = Set(Utc::now());

    Ok(todo.update(db).await?)
}
