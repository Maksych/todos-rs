use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, TypedHeader,
};
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::{actions, security};

#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub joined_at: DateTime<Utc>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| {
                    tracing::error!("{err}");

                    (StatusCode::BAD_REQUEST, "Invalid token")
                })?;

        let user_id = security::verify_access_token(token.token().to_owned())
            .await
            .map_err(|err| {
                tracing::error!("{err}");

                (StatusCode::UNAUTHORIZED, "Invalid or expired token")
            })?;

        let Extension(db) = Extension::<DatabaseConnection>::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                tracing::error!("{err}");

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            })?;

        let user = actions::get_user_by_id(&db, &user_id)
            .await
            .map_err(|err| {
                tracing::error!("{err}");

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            })?;

        Ok(Self {
            id: user.id,
            username: user.username,
            joined_at: user.joined_at,
        })
    }
}
