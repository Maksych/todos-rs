use async_trait::async_trait;
use sea_query::{DeleteStatement, SelectStatement};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("SeaQuery: {0}")]
    SeaQuery(#[from] sea_query::error::Error),
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
#[async_trait]
pub trait Repository<'a> {
    type Model: Clone;

    fn new(db: &'a PgPool) -> Self;

    async fn select<F>(&self, f: F) -> RepositoryResult<Vec<Self::Model>>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn count<F>(&self, f: F) -> RepositoryResult<i64>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn insert(&self, item: Self::Model) -> RepositoryResult<Self::Model>;

    async fn get<F>(&self, f: F) -> RepositoryResult<Option<Self::Model>>
    where
        F: FnOnce(&mut SelectStatement) + Send,
    {
        Ok(self.select(f).await?.pop())
    }

    async fn get_by_id(&self, id: &Uuid) -> RepositoryResult<Option<Self::Model>>;

    async fn update(&self, item: Self::Model) -> RepositoryResult<Self::Model>;

    async fn delete_by_id(&self, id: &Uuid) -> RepositoryResult<()>;

    async fn delete<F>(&self, f: F) -> RepositoryResult<()>
    where
        F: FnOnce(&mut DeleteStatement) + Send;
}
