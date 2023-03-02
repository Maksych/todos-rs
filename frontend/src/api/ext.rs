use serde::de::DeserializeOwned;

use super::error::{ApiError, FieldError};

pub trait FieldErrorsMessagesExt {
    fn messages(&self) -> Vec<String>;
}

impl FieldErrorsMessagesExt for Vec<FieldError> {
    fn messages(&self) -> Vec<String> {
        self.iter().map(|err| err.message.clone()).collect()
    }
}

pub trait ApiErrorOptionExt {
    fn json<T>(&self) -> T
    where
        T: Default + DeserializeOwned;
}

impl ApiErrorOptionExt for Option<ApiError> {
    fn json<T>(&self) -> T
    where
        T: Default + DeserializeOwned,
    {
        match self {
            Some(err) => err.json(),
            None => T::default(),
        }
    }
}
