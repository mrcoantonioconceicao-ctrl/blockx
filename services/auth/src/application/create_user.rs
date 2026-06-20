use shared::ids::generate_id;

use crate::application::password_service;
use crate::domain::user::User;

use crate::infrastructure::user_repository::UserRepository;

pub fn execute<T: UserRepository>(
    repository: &T,
    email: String,
    password: String,
) -> Result<User, String> {

    if email.trim().is_empty() {
        return Err(
            "email cannot be empty"
                .to_string(),
        );
    }

    if !email.contains("@") {
        return Err(
            "invalid email"
                .to_string(),
        );
    }

    if password.len() < 6 {
        return Err(
            "password too short"
                .to_string(),
        );
    }

    if repository.exists(&email) {

        return Err(
            "user already exists"
                .to_string(),
        );
    }

    let password_hash =
        password_service::hash_password(
            &password,
        );

    let user = User {
        id: generate_id(),
        email,
        password_hash,
    };

    Ok(user)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::infrastructure::in_memory_user_repository::InMemoryUserRepository;

    use crate::infrastructure::user_repository::UserRepository;

    #[test]
    fn should_create_user() {

        let repository =
            InMemoryUserRepository::new();

        let user = execute(
            &repository,
            "admin@blockx.io".to_string(),
            "123456".to_string(),
        );

        assert!(user.is_ok());
    }

    #[test]
    fn should_reject_empty_email() {

        let repository =
            InMemoryUserRepository::new();

        let user = execute(
            &repository,
            "".to_string(),
            "123456".to_string(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn should_reject_invalid_email() {

        let repository =
            InMemoryUserRepository::new();

        let user = execute(
            &repository,
            "admin".to_string(),
            "123456".to_string(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn should_reject_short_password() {

        let repository =
            InMemoryUserRepository::new();

        let user = execute(
            &repository,
            "admin@blockx.io".to_string(),
            "123".to_string(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn should_reject_duplicate_user() {

        let repository =
            InMemoryUserRepository::new();

        let user =
            execute(
                &repository,
                "admin@blockx.io".to_string(),
                "123456".to_string(),
            )
            .unwrap();

        repository.save(&user);

        let duplicated =
            execute(
                &repository,
                "admin@blockx.io".to_string(),
                "654321".to_string(),
            );

        assert!(duplicated.is_err());
    }
}
