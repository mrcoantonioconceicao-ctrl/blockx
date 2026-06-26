use std::sync::Arc;

use crate::infrastructure::{
    in_memory_user_repository::InMemoryUserRepository,
    refresh_token_store::RefreshTokenStore,
};

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<InMemoryUserRepository>,
    pub refresh_store: Arc<RefreshTokenStore>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(InMemoryUserRepository::new()),
            refresh_store: Arc::new(RefreshTokenStore::new()),
        }
    }
}
