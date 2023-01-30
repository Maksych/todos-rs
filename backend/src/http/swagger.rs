use axum::{response::Html, routing::get, Json, Router};

pub async fn create_router() -> Result<Router, anyhow::Error> {
    let swagger_html = include_str!("../../swagger/index.html");

    let openapi_json =
        serde_yaml::from_str::<serde_json::Value>(include_str!("../../swagger/openapi.yaml"))?;

    let router = Router::new()
        .route("/", get(move || async move { Html(swagger_html) }))
        .route(
            "/openapi.json",
            get(move || async move { Json(openapi_json) }),
        );

    Ok(router)
}
