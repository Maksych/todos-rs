use tokio::task;

use super::Error;

pub async fn hash_password(password: String) -> Result<String, Error> {
    Ok(task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST)).await??)
}

pub async fn verify_password(password: String, hashed_password: String) -> Result<bool, Error> {
    Ok(task::spawn_blocking(move || bcrypt::verify(password, &hashed_password)).await??)
}
