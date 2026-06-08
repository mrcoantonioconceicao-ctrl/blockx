use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::user::User;

use super::user_repository::UserRepository;

pub struct InMemoryUserRepository {
    users: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, user: &User) {
        let mut users =
            self.users.lock().unwrap();

        users.insert(
            user.email.clone(),
            user.clone(),
        );

        println!(
            "User saved in memory: {:?}",
            user
        );
    }

    fn find_by_email(
        &self,
        email: &str,
    ) -> Option<User> {
        let users =
            self.users.lock().unwrap();

        users.get(email).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_save_and_find_user() {
        let repository =
            InMemoryUserRepository::new();

        let user = User {
            id: "1".to_string(),
            email: "admin@blockx.io".to_string(),
            password_hash: "hash".to_string(),
        };

        repository.save(&user);

        let found =
            repository.find_by_email(
                "admin@blockx.io",
            );

        assert!(found.is_some());

        let found = found.unwrap();

        assert_eq!(
            found.email,
            "admin@blockx.io"
        );

        assert_eq!(
            found.id,
            "1"
        );
    }
}
