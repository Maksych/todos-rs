use axum::{
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Extension, Json, TypedHeader,
};
use sqlx::PgPool;
use validator::Validate;

use crate::{actions, models, security};

use super::Error;

pub async fn sign_up(
    Extension(db): Extension<PgPool>,
    Json(credentials): Json<models::Credentials>,
) -> Result<impl IntoResponse, Error> {
    credentials.validate()?;

    let user = actions::create_user(&db, &credentials.username, &credentials.password).await?;

    Ok(Json(security::create_token(user).await?))
}

pub async fn sign_in(
    Extension(db): Extension<PgPool>,
    Json(credentials): Json<models::Credentials>,
) -> Result<impl IntoResponse, Error> {
    credentials.validate()?;

    let user = actions::get_user_by_username(&db, &credentials.username)
        .await
        .map_err(|_| Error::InvalidCredentials)?;

    if !security::verify_password(credentials.password, user.hashed_password.clone()).await? {
        return Err(Error::InvalidCredentials);
    }

    Ok(Json(security::create_token(user).await?))
}

pub async fn sign_refresh(
    Extension(db): Extension<PgPool>,
    TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    let claims = security::verify_refresh_token(token.token().to_string()).await?;

    let user = actions::get_user_by_id(&db, &claims.sub).await?;

    if user.sid != claims.sid {
        return Err(Error::InvalidToken);
    }

    Ok(Json(security::create_token(user).await?))
}
