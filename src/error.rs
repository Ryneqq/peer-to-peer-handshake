use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Bitcoin encode error occurred: {0}")]
    Encode(#[from] bitcoin::consensus::encode::Error),

    #[error("A custom error occurred: {0}")]
    CustomError(String),
}
