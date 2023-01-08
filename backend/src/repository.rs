use async_trait::async_trait;
use sea_query::SelectStatement;
use sqlx::PgPool;
use uuid::Uuid;

pub use error::Error;
pub use todo::TodoRepo;
pub use user::UserRepo;

mod error;
mod todo;
mod user;

#[async_trait]
pub trait Repo<'a> {
    type Model: Clone;

    fn new(db: &'a PgPool) -> Self;

    async fn select<F>(&self, f: F) -> Result<Vec<Self::Model>, Error>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn count<F>(&self, f: F) -> Result<i64, Error>
    where
        F: FnOnce(&mut SelectStatement) + Send;

    async fn insert(&self, item: Self::Model) -> Result<Self::Model, Error>;

    async fn get<F>(&self, f: F) -> Result<Self::Model, Error>
    where
        F: FnOnce(&mut SelectStatement) + Send,
    {
        match self.select(f).await?.get(0) {
            Some(item) => Ok(item.clone()),
            None => Err(Error::NotFound),
        }
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Self::Model, Error>;

    async fn update(&self, item: Self::Model) -> Result<Self::Model, Error>;

    async fn delete(&self, item: Self::Model) -> Result<(), Error>;
}
