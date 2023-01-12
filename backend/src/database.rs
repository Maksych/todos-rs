use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn get_db() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await?)
}
