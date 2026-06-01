use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("authentication failed")]
    Authentication,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("resource not found")]
    NotFound,

    #[error("database error")]
    Database,

    #[error("internal server error")]
    Internal,
}
