use std::sync::Arc;
use crate::infrastructure::in_memory_user_repository::InMemoryUserRepository;

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<InMemoryUserRepository>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(InMemoryUserRepository::new()),
        }
    }
}
