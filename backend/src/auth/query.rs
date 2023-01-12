pub use sea_query::*;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    HashedPassword,
    JoinedAt,
}
