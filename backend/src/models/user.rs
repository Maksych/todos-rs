use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub sid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 10))]
    pub password: String,
    #[validate(length(min = 10))]
    pub new_password: String,
}
