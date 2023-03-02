use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TodosQuery {
    pub is_completed: Option<bool>,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TodosDeleteQuery {
    pub is_completed: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewTodo {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameTodo {
    pub name: String,
}
