use std::env;

use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use tokio::task;
use uuid::Uuid;

use crate::models::{auth::Token, user::User};

use super::Error;

static ACCESS_EXPIRE_SECONDS: u64 = 60 * 15;
static REFRESH_EXPIRE_SECONDS: u64 = 60 * 60 * 24;

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

pub async fn create_token(user: User) -> Result<Token, Error> {
    task::spawn_blocking(move || {
        let header = jwt::Header::default();

        let access_claims = Claims::new(user.id, Audience::Access, ACCESS_EXPIRE_SECONDS);

        let refresh_claims = Claims::new(user.id, Audience::Refresh, REFRESH_EXPIRE_SECONDS);

        let key = get_encoding_key()?;

        Ok(Token {
            access: jwt::encode(&header, &access_claims, &key)?,
            refresh: jwt::encode(&header, &refresh_claims, &key)?,
        })
    })
    .await?
}

pub fn get_encoding_key() -> Result<jwt::EncodingKey, Error> {
    let secret = env::var("SECRET")?;

    Ok(jwt::EncodingKey::from_secret(secret.as_bytes()))
}

pub async fn verify_token(token: String, aud: Audience) -> Result<Uuid, Error> {
    task::spawn_blocking(move || {
        let key = get_decoding_key()?;

        let validation = jwt::Validation::default();

        let token_data = jwt::decode::<Claims>(&token, &key, &validation)?;

        if token_data.claims.aud == aud {
            Ok(token_data.claims.sub)
        } else {
            Err(Error::JwtInvalidAudience)
        }
    })
    .await?
}

pub fn get_decoding_key() -> Result<jwt::DecodingKey, Error> {
    let secret = env::var("SECRET")?;

    Ok(jwt::DecodingKey::from_secret(secret.as_bytes()))
}

pub async fn verify_access_token(token: String) -> Result<Uuid, Error> {
    verify_token(token, Audience::Access).await
}

pub async fn verify_refresh_token(token: String) -> Result<Uuid, Error> {
    verify_token(token, Audience::Refresh).await
}
