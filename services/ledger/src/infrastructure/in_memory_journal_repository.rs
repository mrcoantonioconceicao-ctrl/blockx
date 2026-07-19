use std::sync::{Arc, Mutex};

use crate::domain::Journal;

use super::journal_repository::JournalRepository;

#[derive(Clone)]
pub struct InMemoryJournalRepository {
    journals: Arc<Mutex<Vec<Journal>>>,
}

impl InMemoryJournalRepository {
    pub fn new() -> Self {
        Self {
            journals: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl JournalRepository for InMemoryJournalRepository {
    fn save(&self, journal: Journal) {
        self.journals.lock().unwrap().push(journal);
    }

    fn find_by_id(&self, id: &str) -> Option<Journal> {
        self.journals
            .lock()
            .unwrap()
            .iter()
            .find(|j| j.id.to_string() == id)
            .cloned()
    }

    fn list(&self) -> Vec<Journal> {
        self.journals.lock().unwrap().clone()
    }
}
