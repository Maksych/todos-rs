use std::net::SocketAddr;

use axum::{
    http::{header, Method},
    Extension, Router, Server,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod actions;
mod database;
mod handlers;
mod models;
mod query;
mod repository;
mod security;

pub async fn create_app() -> anyhow::Result<Router> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::new()
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    let db = database::get_db().await?;

    let app = Router::new()
        .nest("/api/v1", handlers::create_router().await?)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(Extension(db));

    Ok(app)
}

pub async fn serve(app: Router, addr: &SocketAddr) -> anyhow::Result<()> {
    tracing::info!("Listening on {}", addr);
    Server::bind(addr).serve(app.into_make_service()).await?;
    Ok(())
}
