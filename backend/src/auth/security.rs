use std::env;

use bcrypt::BcryptError;
use jsonwebtoken as jwt;
use once_cell::sync;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::{self, JoinError};
use uuid::Uuid;

pub static SECRET: sync::Lazy<String> =
    sync::Lazy::new(|| env::var("SECRET").expect("Environment variable \"SECRET\" not found"));

pub static ACCESS_EXPIRE_SECONDS: sync::Lazy<u64> =
    sync::Lazy::new(|| match env::var("ACCESS_EXPIRE_SECONDS") {
        Ok(value) => value
            .parse::<u64>()
            .expect("Environment variable \"ACCESS_EXPIRE_SECONDS\" is not integer"),
        Err(_) => 60 * 15,
    });

pub static REFRESH_EXPIRE_SECONDS: sync::Lazy<u64> =
    sync::Lazy::new(|| match env::var("REFRESH_EXPIRE_SECONDS") {
        Ok(value) => value
            .parse::<u64>()
            .expect("Environment variable \"REFRESH_EXPIRE_SECONDS\" is not integer"),
        Err(_) => 60 * 60 * 24,
    });

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Bcrypt {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("Join {0}")]
    Join(#[from] JoinError),
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

        let access_claims = Claims::new(user_id, Audience::Access, *ACCESS_EXPIRE_SECONDS);

        let refresh_claims = Claims::new(user_id, Audience::Refresh, *REFRESH_EXPIRE_SECONDS);

        let key = jwt::EncodingKey::from_secret(SECRET.as_bytes());

        Ok(Token {
            access: jwt::encode(&header, &access_claims, &key)?,
            refresh: jwt::encode(&header, &refresh_claims, &key)?,
        })
    })
    .await?
}

pub async fn verify_token(token: String, aud: Audience) -> Result<Uuid, SecurityError> {
    task::spawn_blocking(move || {
        let key = jwt::DecodingKey::from_secret(SECRET.as_bytes());

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

pub async fn verify_access_token(token: String) -> Result<Uuid, SecurityError> {
    verify_token(token, Audience::Access).await
}

pub async fn verify_refresh_token(token: String) -> Result<Uuid, SecurityError> {
    verify_token(token, Audience::Refresh).await
}
