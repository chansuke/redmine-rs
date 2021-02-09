use thiserror::Error;

#[derive(Error, Debug)]
pub enum RmError {
    #[error("failed to deserialize an instance from a string of JSON text: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("failed to read environment variable:")]
    VarError(#[from] std::env::VarError),
    #[error("failed to format the given variables: {0}")]
    FormatError(#[from] std::fmt::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("failed to get a successful response: {0}")]
    Reqwest(#[from] reqwest::Error),
}
