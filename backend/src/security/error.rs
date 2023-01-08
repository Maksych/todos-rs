use std::env::VarError;

use bcrypt::BcryptError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bcrypt {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("Join {0}")]
    Join(#[from] JoinError),
    #[error("Var: {0}")]
    Var(#[from] VarError),
    #[error("Jwt: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("JwtInvalidAudience")]
    JwtInvalidAudience,
}
