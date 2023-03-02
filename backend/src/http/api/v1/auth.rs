use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use validator::Validate;

use crate::{
    auth::{
        actions::{self, ActionError},
        security::SecurityError,
    },
    http::extractors::AuthUser,
    repository::RepositoryError,
};

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Action: {0}")]
    Action(#[from] ActionError),
    #[error("Validation: {0}")]
    Validation(#[from] validator::ValidationErrors),
}

fn action_into_response(error: ActionError) -> Response {
    match error {
        ActionError::Repository(inner) => repo_into_response(inner),
        ActionError::Security(inner) => security_into_response(inner),
        ActionError::UserAlredyExists => {
            (StatusCode::BAD_REQUEST, error.to_string()).into_response()
        }
        ActionError::InvalidCredentials => {
            (StatusCode::UNAUTHORIZED, error.to_string()).into_response()
        }
        ActionError::NotFound => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

fn repo_into_response(error: RepositoryError) -> Response {
    tracing::error!("{}", error);

    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
}

fn validation_into_response(error: validator::ValidationErrors) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(error)).into_response()
}

fn security_into_response(error: SecurityError) -> Response {
    match error {
        SecurityError::JwtInvalidAudience => {
            (StatusCode::UNAUTHORIZED, error.to_string()).into_response()
        }
        _ => {
            tracing::error!("{}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::Action(inner) => action_into_response(inner),
            HandlerError::Validation(inner) => validation_into_response(inner),
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct Credentials {
    #[validate(length(min = 5, message = "Too short"))]
    pub username: String,
    #[validate(length(min = 10, message = "Too short"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct Token {
    pub access: String,
    pub refresh: String,
}

#[derive(Deserialize, Validate)]
pub struct PasswordChange {
    #[validate(length(min = 10, message = "Too short"))]
    pub password: String,
    #[validate(length(min = 10, message = "Too short"))]
    pub new_password: String,
}

pub async fn sign_up(
    Extension(db): Extension<PgPool>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, HandlerError> {
    credentials.validate()?;

    Ok(Json(
        actions::sign_up(&db, &credentials.username, &credentials.password).await?,
    ))
}

pub async fn sign_in(
    Extension(db): Extension<PgPool>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, HandlerError> {
    credentials.validate()?;

    Ok(Json(
        actions::sign_in(&db, &credentials.username, &credentials.password).await?,
    ))
}

pub async fn sign_refresh(
    Extension(db): Extension<PgPool>,
    TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(
        actions::sign_refresh(&db, token.token().to_string()).await?,
    ))
}

pub async fn profile(user: AuthUser) -> impl IntoResponse {
    Json(user)
}

pub async fn change_password(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Json(payload): Json<PasswordChange>,
) -> Result<impl IntoResponse, HandlerError> {
    payload.validate()?;

    actions::change_password(&db, &user.id, &payload.password, &payload.new_password).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_router() -> anyhow::Result<Router> {
    Ok(Router::new()
        .route("/sign-in", post(sign_in))
        .route("/sign-up", post(sign_up))
        .route("/sign-refresh", post(sign_refresh))
        .route("/profile", get(profile))
        .route("/change-password", post(change_password)))
}
