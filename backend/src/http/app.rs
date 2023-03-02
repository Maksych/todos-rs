use axum::{
    http::{header, Method},
    Extension, Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use super::{api, swagger};
use crate::database;

pub async fn create_app() -> anyhow::Result<Router> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::new()
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    let db = database::get_db().await?;

    let api_router = api::create_router().await?;

    let swagger_router = swagger::create_router().await?;

    let app = Router::new()
        .nest("/api", api_router)
        .nest("/swagger", swagger_router)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(Extension(db));

    Ok(app)
}
