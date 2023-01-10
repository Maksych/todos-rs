use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::user::User,
    repository::{Repo, UserRepo},
    security,
};

use super::Error;

pub async fn get_user_by_id(db: &PgPool, id: &Uuid) -> Result<User, Error> {
    Ok(UserRepo::new(db).get_by_id(id).await?)
}

pub async fn get_user_by_username(db: &PgPool, username: &str) -> Result<User, Error> {
    Ok(UserRepo::new(db).get_by_username(username).await?)
}

pub async fn create_user(db: &PgPool, username: &str, password: &str) -> Result<User, Error> {
    let repo = UserRepo::new(db);

    if repo.get_by_username(username).await.is_ok() {
        return Err(Error::UserAlredyExists);
    }

    let user = User {
        id: Uuid::new_v4(),
        username: username.to_owned(),
        hashed_password: security::hash_password(password.to_owned()).await?,
        joined_at: Utc::now(),
    };

    Ok(repo.insert(user).await?)
}

pub async fn change_password(db: &PgPool, id: &Uuid, password: &str) -> Result<User, Error> {
    let repo = UserRepo::new(db);

    let mut user = repo.get_by_id(id).await?;

    user.hashed_password = security::hash_password(password.into()).await?;

    Ok(repo.update(user).await?)
}
