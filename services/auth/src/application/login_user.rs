use crate::application::jwt_service::generate_token;
use crate::application::refresh_token_service::RefreshTokenService;
use crate::domain::user::User;
use crate::application::auth_error::AuthError;

#[derive(Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn execute(
    user: &User,
) -> Result<LoginResponse, AuthError> {

    let access_token = generate_token(&user.id.to_string())
        .map_err(|_| AuthError::TokenGenerationFailed)?;

    let refresh_service = RefreshTokenService::new();
    let refresh_token = refresh_service.generate_refresh_token(&user.id.to_string());

    Ok(LoginResponse {
        access_token,
        refresh_token: refresh_token.token,
    })
}
