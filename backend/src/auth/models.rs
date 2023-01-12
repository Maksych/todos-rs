use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub joined_at: DateTime<Utc>,
}
