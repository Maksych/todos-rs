use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use thiserror::Error;

use crate::store::{Action, Store};

#[derive(Debug, Clone, Error)]
pub enum ApiError {
    #[error("TokenExpired")]
    TokenExpired,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("UnprocessableEntity")]
    UnprocessableEntity(Value),
    #[error("{0}")]
    Reqwest(String),
}

impl ApiError {
    pub fn json<T>(&self) -> T
    where
        T: Default + DeserializeOwned,
    {
        match self {
            ApiError::UnprocessableEntity(value) => match serde_json::from_value(value.clone()) {
                Ok(value) => value,
                Err(err) => {
                    Store::dispatch(Action::AlertError(err.to_string()));

                    T::default()
                }
            },
            _ => T::default(),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::Reqwest(value.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldError {
    pub message: String,
}
