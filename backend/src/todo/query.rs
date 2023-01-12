pub use sea_query::*;

#[derive(Iden)]
pub enum Todo {
    Table,
    Id,
    UserId,
    Name,
    IsDone,
    CreatedAt,
    UpdatedAt,
    DoneAt,
}
