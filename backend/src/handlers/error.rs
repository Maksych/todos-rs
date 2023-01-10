use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::{actions, repository, security};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Action: {0}")]
    Action(#[from] actions::Error),
    #[error("Validation: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Security: {0}")]
    Security(#[from] security::Error),
    #[error("InvalidCredentials")]
    InvalidCredentials,
}

fn action_into_response(error: actions::Error) -> Response {
    use actions::Error;

    match error {
        Error::Repo(inner) => repo_into_response(inner),
        Error::Security(inner) => security_into_response(inner),
        Error::Forbidden => (StatusCode::FORBIDDEN, error.to_string()).into_response(),
        Error::UserAlredyExists => (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
    }
}

fn repo_into_response(error: repository::Error) -> Response {
    use repository::Error;

    match error {
        Error::NotFound => StatusCode::NOT_FOUND.into_response(),
        _ => {
            tracing::error!("{}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

fn validation_into_response(error: validator::ValidationErrors) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(error)).into_response()
}

fn security_into_response(error: security::Error) -> Response {
    use security::Error;

    match error {
        Error::JwtInvalidAudience => (StatusCode::UNAUTHORIZED, error.to_string()).into_response(),
        _ => {
            tracing::error!("{}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Action(inner) => action_into_response(inner),
            Error::Validation(inner) => validation_into_response(inner),
            Error::Security(inner) => security_into_response(inner),
            Error::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
        }
    }
}
