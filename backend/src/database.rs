use std::env;

use once_cell::sync;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub static DATABASE_URL: sync::Lazy<String> = sync::Lazy::new(|| {
    env::var("DATABASE_URL").expect("Environment variable \"DATABASE_URL\" not found")
});

pub async fn get_db() -> anyhow::Result<DatabaseConnection> {
    let opt = ConnectOptions::new(DATABASE_URL.clone());

    Ok(Database::connect(opt).await?)
}
