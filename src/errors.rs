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
