use std::{env, net::SocketAddr};

use backend::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = http::app::create_app().await?;

    let addr: SocketAddr = env::var("ADDR")?.parse()?;

    http::server::serve(app, &addr).await
}
