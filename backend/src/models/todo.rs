use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct NewTodo {
    #[validate(length(min = 5))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RenameTodo {
    #[validate(length(min = 5))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TodosQuery {
    #[validate(range(min = 1, max = 25))]
    #[serde(default = "TodosQuery::default_limit")]
    pub limit: u64,
    #[validate(range(min = 0))]
    #[serde(default = "TodosQuery::default_offset")]
    pub offset: u64,
}

impl TodosQuery {
    fn default_limit() -> u64 {
        10
    }

    fn default_offset() -> u64 {
        0
    }
}
