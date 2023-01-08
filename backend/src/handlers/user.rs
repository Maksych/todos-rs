use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    actions,
    models::{auth::AuthUser, user::PasswordChange},
    security,
};

use super::Error;

pub async fn profile(user: AuthUser) -> impl IntoResponse {
    Json(user)
}

pub async fn change_password(
    Extension(db): Extension<PgPool>,
    user: AuthUser,
    Json(payload): Json<PasswordChange>,
) -> Result<impl IntoResponse, Error> {
    payload.validate()?;

    let user = actions::get_user_by_id(&db, &user.id).await?;

    if !security::verify_password(payload.password.clone(), user.hashed_password.clone()).await? {
        return Err(Error::InvalidCredentials);
    }

    actions::change_password(&db, &user.id, &payload.new_password).await?;

    Ok(StatusCode::NO_CONTENT)
}
