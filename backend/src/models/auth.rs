use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, TypedHeader,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{actions, security};

#[derive(Debug, Deserialize, Validate)]
pub struct Credentials {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(length(min = 10))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub joined_at: DateTime<Utc>,
}

#[axum::async_trait]
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
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid token"))?;

        let Extension(db) = Extension::<PgPool>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error from request db pool",
                )
            })?;

        let claims = security::verify_access_token(token.token().to_owned())
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        let user = actions::get_user_by_id(&db, &claims.sub)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error get user by id"))?;

        if user.sid != claims.sid {
            return Err((StatusCode::UNAUTHORIZED, "Invalid or expired token"));
        }

        Ok(Self {
            id: user.id,
            username: user.username,
            joined_at: user.joined_at,
        })
    }
}
