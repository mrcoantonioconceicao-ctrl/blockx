use crate::application::create_user;
use crate::application::jwt_service::generate_token;
use crate::application::password_service::verify_password;

use crate::infrastructure::user_repository::UserRepository;

pub fn execute<T: UserRepository>(
    repository: &T,
    email: &str,
    password: &str,
) -> Option<String> {

    let user =
        repository.find_by_email(email)?;

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

    use crate::infrastructure::in_memory_user_repository::InMemoryUserRepository;

    use crate::infrastructure::user_repository::UserRepository;

    #[test]
    fn should_login_with_correct_password() {

        let repository =
            InMemoryUserRepository::new();

        let user =
            create_user::execute(
                &repository,
                "admin@blockx.io".to_string(),
                "123456".to_string(),
            )
            .unwrap();

        repository.save(&user);

        let token =
            execute(
                &repository,
                "admin@blockx.io",
                "123456",
            );

        assert!(
            token.is_some()
        );
    }

    #[test]
    fn should_reject_invalid_password() {

        let repository =
            InMemoryUserRepository::new();

        let user =
            create_user::execute(
                &repository,
                "admin@blockx.io".to_string(),
                "123456".to_string(),
            )
            .unwrap();

        repository.save(&user);

        let token =
            execute(
                &repository,
                "admin@blockx.io",
                "wrong-password",
            );

        assert!(
            token.is_none()
        );
    }

    #[test]
    fn should_reject_unknown_user() {

        let repository =
            InMemoryUserRepository::new();

        let token =
            execute(
                &repository,
                "unknown@blockx.io",
                "123456",
            );

        assert!(
            token.is_none()
        );
    }
}
