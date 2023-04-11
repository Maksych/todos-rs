use std::{env, net::SocketAddr};

use backend::http;
use once_cell::sync;

static ADDR: sync::Lazy<SocketAddr> = sync::Lazy::new(|| {
    env::var("ADDR")
        .expect("Environment variable \"ADDR\" not found")
        .parse::<SocketAddr>()
        .expect("Environment variable \"ADDR\" is invalid")
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = http::app::create_app().await?;

    http::server::serve(app, &ADDR).await
}
