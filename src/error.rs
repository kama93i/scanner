use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection {0}:{1} refused")]
    ConnectionError(String, u16),

    #[error("Invalid Input: {0}")]
    InputError(String),
}
