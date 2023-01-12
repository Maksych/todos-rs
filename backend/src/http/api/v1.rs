use axum::Router;

mod auth;
mod todo;

pub async fn create_router() -> anyhow::Result<Router> {
    Ok(Router::new()
        .merge(auth::create_router().await?)
        .merge(todo::create_router().await?))
}
