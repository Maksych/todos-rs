use axum::{
    routing::{get, post},
    Router,
};

pub use error::Error;

mod auth;
mod error;
mod todo;
mod user;

pub async fn create_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/sign-up", post(auth::sign_up))
        .route("/sign-in", post(auth::sign_in))
        .route("/sign-refresh", post(auth::sign_refresh))
        .route("/profile", get(user::profile))
        .route("/change-password", post(user::change_password))
        .route("/todos", get(todo::get_todos).post(todo::create_todo))
        .route("/todos-active", get(todo::get_active_todos))
        .route("/todos-completed", get(todo::get_completed_todos))
        .route(
            "/todos/:id",
            get(todo::get_todo)
                .patch(todo::rename_todo)
                .delete(todo::delete_todo),
        )
        .route("/todos/:id/done", post(todo::done_todo))
        .route("/todos/:id/revert", post(todo::revert_todo));
    Ok(router)
}
