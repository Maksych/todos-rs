use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

static SWAGGER_HTML: &str = include_str!("../../swagger/index.html");

static OPENAPI_YAML: &str = include_str!("../../swagger/openapi.yaml");

async fn swagger_ui() -> impl IntoResponse {
    Html(SWAGGER_HTML)
}

async fn openapi_yaml() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/yaml")], OPENAPI_YAML)
}

pub async fn create_router() -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .route("/", get(swagger_ui))
        .route("/openapi.yaml", get(openapi_yaml));

    Ok(router)
}
