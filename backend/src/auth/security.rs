use std::env::{self, VarError};

use bcrypt::BcryptError;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::{self, JoinError};
use uuid::Uuid;

static ACCESS_EXPIRE_SECONDS: u64 = 60 * 15;
static REFRESH_EXPIRE_SECONDS: u64 = 60 * 60 * 24;

#[derive(Debug, Error)]
pub enum SecurityError {
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

pub async fn hash_password(password: String) -> Result<String, SecurityError> {
    Ok(task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST)).await??)
}

pub async fn verify_password(
    password: String,
    hashed_password: String,
) -> Result<bool, SecurityError> {
    Ok(task::spawn_blocking(move || bcrypt::verify(password, &hashed_password)).await??)
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub access: String,
    pub refresh: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub aud: Audience,
    pub exp: u64,
    pub iat: u64,
    pub nbf: u64,
    pub sub: Uuid,
}

impl Claims {
    fn new(sub: Uuid, aud: Audience, exp: u64) -> Self {
        let now = jwt::get_current_timestamp();

        Self {
            aud,
            exp: now + exp,
            iat: now,
            nbf: now,
            sub,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Audience {
    Access,
    Refresh,
}

pub async fn create_token(user_id: Uuid) -> Result<Token, SecurityError> {
    task::spawn_blocking(move || {
        let header = jwt::Header::default();

        let access_claims = Claims::new(user_id, Audience::Access, ACCESS_EXPIRE_SECONDS);

        let refresh_claims = Claims::new(user_id, Audience::Refresh, REFRESH_EXPIRE_SECONDS);

        let key = get_encoding_key()?;

        Ok(Token {
            access: jwt::encode(&header, &access_claims, &key)?,
            refresh: jwt::encode(&header, &refresh_claims, &key)?,
        })
    })
    .await?
}

pub fn get_encoding_key() -> Result<jwt::EncodingKey, SecurityError> {
    let secret = env::var("SECRET")?;

    Ok(jwt::EncodingKey::from_secret(secret.as_bytes()))
}

pub async fn verify_token(token: String, aud: Audience) -> Result<Uuid, SecurityError> {
    task::spawn_blocking(move || {
        let key = get_decoding_key()?;

        let validation = jwt::Validation::default();

        let token_data = jwt::decode::<Claims>(&token, &key, &validation)?;

        if token_data.claims.aud == aud {
            Ok(token_data.claims.sub)
        } else {
            Err(SecurityError::JwtInvalidAudience)
        }
    })
    .await?
}

pub fn get_decoding_key() -> Result<jwt::DecodingKey, SecurityError> {
    let secret = env::var("SECRET")?;

    Ok(jwt::DecodingKey::from_secret(secret.as_bytes()))
}

pub async fn verify_access_token(token: String) -> Result<Uuid, SecurityError> {
    verify_token(token, Audience::Access).await
}

pub async fn verify_refresh_token(token: String) -> Result<Uuid, SecurityError> {
    verify_token(token, Audience::Refresh).await
}
