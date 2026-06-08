use crate::application::password_service::verify_password;
use crate::domain::user::User;

pub fn execute(
    user: &User,
    password: &str,
) -> bool {
    verify_password(
        password,
        &user.password_hash,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::create_user;

    #[test]
    fn should_login_with_correct_password() {
        let user = create_user::execute(
            "admin@blockx.io".to_string(),
            "123456".to_string(),
        );

        assert!(
            execute(
                &user,
                "123456",
            )
        );
    }

    #[test]
    fn should_reject_invalid_password() {
        let user = create_user::execute(
            "admin@blockx.io".to_string(),
            "123456".to_string(),
        );

        assert!(
            !execute(
                &user,
                "wrong-password",
            )
        );
    }
}
