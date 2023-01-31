use axum::{response::Html, routing::get, Json, Router};

static SWAGGER_HTML: &str = include_str!("../../swagger/index.html");

static OPENAPI_YAML: &str = include_str!("../../swagger/openapi.yaml");

pub async fn create_router() -> Result<Router, anyhow::Error> {
    let openapi_json = serde_yaml::from_str::<serde_json::Value>(OPENAPI_YAML)?;

    let router = Router::new()
        .route("/", get(|| async { Html(SWAGGER_HTML) }))
        .route(
            "/openapi.json",
            get(move || async move { Json(openapi_json) }),
        );

    Ok(router)
}
