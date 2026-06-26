#[derive(Debug, Clone)]
pub enum AuthError {
    UserNotFound,
    InvalidCredentials,
    PasswordHashFailed,
    TokenGenerationFailed,
    RepositoryError,
}
