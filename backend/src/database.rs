use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn get_db() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;

    Ok(PgPoolOptions::new().connect(&database_url).await?)
}
