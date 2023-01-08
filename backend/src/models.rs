use serde::Serialize;

pub use auth::{AuthUser, Credentials, Token};
pub use todo::{NewTodo, RenameTodo, Todo, TodosQuery};
pub use user::{PasswordChange, User};

pub mod auth;
pub mod todo;
pub mod user;

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub count: i64,
}
