use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = backend::create_app().await?;

    let addr: SocketAddr = env::var("ADDR")?.parse()?;

    backend::serve(app, &addr).await
}
