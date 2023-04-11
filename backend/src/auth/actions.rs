use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entities::user;

use super::security::{self, SecurityError, Token};

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("DbErr: {0}")]
    Db(#[from] DbErr),
    #[error("Bcrypt {0}")]
    Security(#[from] SecurityError),
    #[error("UserAlredyExists")]
    UserAlredyExists,
    #[error("InvalidCredentials")]
    InvalidCredentials,
    #[error("Not Found")]
    NotFound,
}

pub async fn sign_up(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> Result<Token, ActionError> {
    if user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ActionError::UserAlredyExists);
    }

    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(username.to_owned()),
        hashed_password: Set(security::hash_password(password.to_owned()).await?),
        joined_at: Set(Utc::now()),
    };

    let new_user = new_user.insert(db).await?;

    Ok(security::create_token(new_user.id).await?)
}

pub async fn sign_in(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> Result<Token, ActionError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?
        .ok_or(ActionError::InvalidCredentials)?;

    if !security::verify_password(password.to_owned(), user.hashed_password.clone()).await? {
        return Err(ActionError::InvalidCredentials);
    }

    Ok(security::create_token(user.id).await?)
}

pub async fn change_password(
    db: &DatabaseConnection,
    user_id: &Uuid,
    password: &str,
    new_password: &str,
) -> Result<(), ActionError> {
    let user = user::Entity::find_by_id(user_id.to_owned())
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)?;

    if !security::verify_password(password.to_owned().clone(), user.hashed_password.clone()).await?
    {
        return Err(ActionError::InvalidCredentials);
    }

    let mut user = user.into_active_model();

    user.hashed_password = Set(security::hash_password(new_password.into()).await?);

    user.update(db).await?;

    Ok(())
}

pub async fn sign_refresh(db: &DatabaseConnection, token: String) -> Result<Token, ActionError> {
    let user_id = security::verify_refresh_token(token).await?;

    let user = get_user_by_id(db, &user_id).await?;

    Ok(security::create_token(user.id).await?)
}

pub async fn get_user_by_id(
    db: &DatabaseConnection,
    id: &Uuid,
) -> Result<user::Model, ActionError> {
    user::Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or(ActionError::NotFound)
}
