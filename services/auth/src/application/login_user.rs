use crate::application::jwt_service::generate_token;
use crate::application::password_service::verify_password;
use crate::domain::user::User;

pub fn execute(
    user: &User,
    password: &str,
) -> Option<String> {
    let authenticated =
        verify_password(
            password,
            &user.password_hash,
        );

    if !authenticated {
        return None;
    }

    Some(
        generate_token(&user.id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::application::create_user;

    #[test]
    fn should_login_with_correct_password() {
        let user =
            create_user::execute(
                "admin@blockx.io".to_string(),
                "123456".to_string(),
            );

        let token =
            execute(&user, "123456");

        assert!(token.is_some());
    }

    #[test]
    fn should_reject_invalid_password() {
        let user =
            create_user::execute(
                "admin@blockx.io".to_string(),
                "123456".to_string(),
            );

        let token =
            execute(&user, "wrong-password");

        assert!(token.is_none());
    }
}
