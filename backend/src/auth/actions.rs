use chrono::Utc;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::repository::{Repository, RepositoryError};

use super::{
    models::User,
    repository::UserRepo,
    security::{self, SecurityError, Token},
};

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Repo: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Bcrypt {0}")]
    Security(#[from] SecurityError),
    #[error("UserAlredyExists")]
    UserAlredyExists,
    #[error("InvalidCredentials")]
    InvalidCredentials,
    #[error("Not Found")]
    NotFound,
}

pub async fn sign_up(db: &PgPool, username: &str, password: &str) -> Result<Token, ActionError> {
    let repo = UserRepo::new(db);

    if repo.get_by_username(username).await?.is_some() {
        return Err(ActionError::UserAlredyExists);
    }

    let user = User {
        id: Uuid::new_v4(),
        username: username.to_owned(),
        hashed_password: security::hash_password(password.to_owned()).await?,
        joined_at: Utc::now(),
    };

    let user = repo.insert(user).await?;

    Ok(security::create_token(user).await?)
}

pub async fn sign_in(db: &PgPool, username: &str, password: &str) -> Result<Token, ActionError> {
    let user = UserRepo::new(db)
        .get_by_username(username)
        .await?
        .ok_or_else(|| ActionError::InvalidCredentials)?;

    if !security::verify_password(password.to_owned(), user.hashed_password.clone()).await? {
        return Err(ActionError::InvalidCredentials);
    }

    Ok(security::create_token(user).await?)
}

pub async fn change_password(
    db: &PgPool,
    user_id: &Uuid,
    password: &str,
    new_password: &str,
) -> Result<(), ActionError> {
    let repo = UserRepo::new(db);

    let mut user = repo
        .get_by_id(user_id)
        .await?
        .ok_or_else(|| ActionError::NotFound)?;

    if !security::verify_password(password.to_owned().clone(), user.hashed_password.clone()).await?
    {
        return Err(ActionError::InvalidCredentials);
    }

    user.hashed_password = security::hash_password(new_password.into()).await?;

    repo.update(user).await?;

    Ok(())
}

pub async fn sign_refresh(db: &PgPool, token: String) -> Result<Token, ActionError> {
    let user_id = security::verify_refresh_token(token).await?;

    let user = get_user_by_id(&db, &user_id).await?;

    Ok(security::create_token(user).await?)
}

pub async fn get_user_by_id(db: &PgPool, id: &Uuid) -> Result<User, ActionError> {
    UserRepo::new(db)
        .get_by_id(id)
        .await?
        .ok_or_else(|| ActionError::NotFound)
}
