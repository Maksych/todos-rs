use std::net::SocketAddr;

use axum::{Router, Server};

pub async fn serve(app: Router, addr: &SocketAddr) -> anyhow::Result<()> {
    tracing::info!("Listening on {}", addr);

    Server::bind(addr).serve(app.into_make_service()).await?;

    Ok(())
}
