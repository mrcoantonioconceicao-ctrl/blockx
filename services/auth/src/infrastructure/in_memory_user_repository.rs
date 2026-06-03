use crate::domain::user::User;

use super::user_repository::UserRepository;

pub struct InMemoryUserRepository;

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, user: &User) {
        println!("User saved in memory: {:?}", user);
    }
}
