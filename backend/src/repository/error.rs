use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("SeaQuery: {0}")]
    SeaQuery(#[from] sea_query::error::Error),
    #[error("Not Found")]
    NotFound,
}
