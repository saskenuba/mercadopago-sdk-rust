use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("{0}")]
    CredentialsError(String),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error("Something wrong happened.")]
    GenericError,
}

#[derive(Error, Debug)]
pub enum CreditCardError {}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Item validation error: {0}")]
    ItemError(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiError {
    message: String,
    status: i32,
    error: String,
    cause: Option<Vec<ErrorCause>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorCause {
    pub description: String,
    pub code: String,
}
