use crate::application::password_service::hash_password;
use crate::domain::user::User;
use crate::application::auth_error::AuthError;

pub fn execute(
    email: String,
    password: String,
) -> Result<User, AuthError> {

    let password_hash = hash_password(&password);

    let user = User::new(
        email,
        password_hash,
    );

    Ok(user)
}
