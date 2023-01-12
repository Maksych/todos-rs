use async_trait::async_trait;
use sea_query::SelectStatement;
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
#[async_trait]
pub trait Repository<'a> {
    type Model: Clone;

    fn new(db: &'a PgPool) -> Self;

    async fn select<F>(&self, f: F) -> Result<Vec<Self::Model>, RepositoryError>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn count<F>(&self, f: F) -> Result<i64, RepositoryError>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn insert(&self, item: Self::Model) -> Result<Self::Model, RepositoryError>;

    async fn get<F>(&self, f: F) -> Result<Option<Self::Model>, RepositoryError>
    where
        F: FnOnce(&mut SelectStatement) + Send,
    {
        Ok(self.select(f).await?.pop())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Self::Model>, RepositoryError>;

    async fn update(&self, item: Self::Model) -> Result<Self::Model, RepositoryError>;

    async fn delete(&self, item: Self::Model) -> Result<(), RepositoryError>;
}
