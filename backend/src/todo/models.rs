use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done_at: Option<DateTime<Utc>>,
}
