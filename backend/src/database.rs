use std::env;

use once_cell::sync;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub static DATABASE_URL: sync::Lazy<String> = sync::Lazy::new(|| {
    env::var("DATABASE_URL").expect("Environment variable \"DATABASE_URL\" not found")
});

pub async fn get_db() -> anyhow::Result<PgPool> {
    Ok(PgPoolOptions::new().connect(&DATABASE_URL).await?)
}
