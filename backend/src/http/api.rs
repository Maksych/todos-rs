use axum::Router;

mod v1;

pub async fn create_router() -> anyhow::Result<Router> {
    Ok(Router::new().nest("/v1", v1::create_router().await?))
}
