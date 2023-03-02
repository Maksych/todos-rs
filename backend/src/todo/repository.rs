use async_trait::async_trait;
use sea_query_binder::SqlxBinder;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::{models::Todo, query as q};
use crate::repository::{Repository, RepositoryError};

pub struct TodoRepo<'a> {
    db: &'a PgPool,
}

#[async_trait]
impl<'a> Repository<'a> for TodoRepo<'a> {
    type Model = Todo;

    fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    async fn select<F>(&self, f: F) -> Result<Vec<Self::Model>, RepositoryError>
    where
        F: FnOnce(&mut q::SelectStatement) + Send,
    {
        let mut stmt = q::Query::select();
        stmt.expr(q::Expr::asterisk()).from(q::Todo::Table);

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
        sub_stmt.expr(q::Expr::asterisk()).from(q::Todo::Table);

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
            .into_table(q::Todo::Table)
            .columns([
                q::Todo::Id,
                q::Todo::UserId,
                q::Todo::Name,
                q::Todo::IsCompleted,
                q::Todo::CreatedAt,
                q::Todo::UpdatedAt,
                q::Todo::CompletedAt,
            ])
            .values([
                item.id.into(),
                item.user_id.into(),
                item.name.clone().into(),
                item.is_completed.into(),
                item.created_at.into(),
                item.updated_at.into(),
                item.completed_at.into(),
            ])?
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(item)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Self::Model>, RepositoryError> {
        self.get(|stmt| {
            stmt.and_where(q::Expr::col(q::Todo::Id).eq(*id));
        })
        .await
    }

    async fn update(&self, item: Self::Model) -> Result<Self::Model, RepositoryError> {
        let (sql, values) = q::Query::update()
            .table(q::Todo::Table)
            .values([
                (q::Todo::UserId, item.user_id.into()),
                (q::Todo::Name, item.name.clone().into()),
                (q::Todo::IsCompleted, item.is_completed.into()),
                (q::Todo::CreatedAt, item.created_at.into()),
                (q::Todo::UpdatedAt, item.updated_at.into()),
                (q::Todo::CompletedAt, item.completed_at.into()),
            ])
            .and_where(q::Expr::col(q::Todo::Id).eq(item.id))
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(item)
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), RepositoryError> {
        let (sql, values) = q::Query::delete()
            .from_table(q::Todo::Table)
            .and_where(q::Expr::col(q::Todo::Id).eq(*id))
            .build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(())
    }

    async fn delete<F>(&self, f: F) -> Result<(), RepositoryError>
    where
        F: FnOnce(&mut q::DeleteStatement) + Send,
    {
        let mut stmt = q::Query::delete();
        stmt.from_table(q::Todo::Table);

        f(&mut stmt);

        let (sql, values) = stmt.build_sqlx(q::PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db).await?;

        Ok(())
    }
}
