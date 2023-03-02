pub use sea_query::*;

#[derive(Iden)]
pub enum Todo {
    Table,
    Id,
    UserId,
    Name,
    IsCompleted,
    CreatedAt,
    UpdatedAt,
    CompletedAt,
}
