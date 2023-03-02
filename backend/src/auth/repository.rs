use async_trait::async_trait;
use sea_query_binder::SqlxBinder;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::{models::User, query as q};
use crate::repository::{Repository, RepositoryError};

pub struct UserRepo<'a> {
    db: &'a PgPool,
}

impl<'a> UserRepo<'a> {
    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        self.get(|stmt| {
            stmt.and_where(q::Expr::col(q::User::Username).eq(username));
        })
        .await
    }
}

#[async_trait]
impl<'a> Repository<'a> for UserRepo<'a> {
    type Model = User;

    fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    async fn select<F>(&self, f: F) -> Result<Vec<Self::Model>, RepositoryError>
    where
        F: FnOnce(&mut q::SelectStatement) + Send,
    {
        let mut stmt = q::Query::select();
        stmt.expr(q::Expr::asterisk()).from(q::User::Table);

        f(&mut stmt);

        let (sql, values) = stmt.build_sqlx(q::PostgresQueryBuilder);

        Ok(sqlx::query_as_with(&sql, values).fetch_all(self.db).await?)
    }

    async fn count<F>(&self, f: F) -> Result<i64, RepositoryError>
    where
        F: FnOnce(&mut q::SelectStatement) + Send,
    {
        let mut stmt = q::Query::select();
        stmt.expr(q::Expr::asterisk().count());

        let mut sub_stmt = q::Query::select();
        sub_stmt.expr(q::Expr::asterisk()).from(q::User::Table);

        f(&mut sub_stmt);

        stmt.from_subquery(sub_stmt.take(), q::Alias::new("t"));

        let (sql, values) = stmt.build_sqlx(q::PostgresQueryBuilder);

        Ok(sqlx::query_with(&sql, values)
            .fetch_one(self.db)
            .await?
            .try_get(0)?)
    }

    async fn insert(&self, item: Self::Model) -> Result<Self::Model, RepositoryError> {
        let (sql, values) = q::Query::insert()
            .into_table(q::User::Table)
            .columns([
                q::User::Id,
                q::User::Username,
                q::User::HashedPassword,
                q::User::JoinedAt,
            ])
            .values([
                item.id.into(),
                item.username.clone().into(),
                item.hashed_password.clone().into(),
                item.joined_at.into(),
            ])?
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(item)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Self::Model>, RepositoryError> {
        Ok(self
            .get(|stmt| {
                stmt.and_where(q::Expr::col(q::User::Id).eq(*id));
            })
            .await?)
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, RepositoryError> {
        let (sql, values) = q::Query::update()
            .table(q::User::Table)
            .values([
                (q::User::Username, item.username.clone().into()),
                (q::User::HashedPassword, item.hashed_password.clone().into()),
                (q::User::JoinedAt, item.joined_at.into()),
            ])
            .and_where(q::Expr::col(q::User::Id).eq(item.id))
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(item)
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), RepositoryError> {
        let (sql, values) = q::Query::delete()
            .from_table(q::User::Table)
            .and_where(q::Expr::col(q::User::Id).eq(*id))
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(())
    }

    async fn delete<F>(&self, f: F) -> Result<(), RepositoryError>
    where
        F: FnOnce(&mut q::DeleteStatement) + Send,
    {
        let mut stmt = q::Query::delete();
        stmt.from_table(q::User::Table);

        f(&mut stmt);

        let (sql, values) = stmt.build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(())
    }
}
