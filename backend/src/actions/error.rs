use thiserror::Error;

use crate::{repository, security};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Repo: {0}")]
    Repo(#[from] repository::Error),
    #[error("Bcrypt {0}")]
    Security(#[from] security::Error),
    #[error("Forbidden")]
    Forbidden,
    #[error("UserAlredyExists")]
    UserAlredyExists,
}
