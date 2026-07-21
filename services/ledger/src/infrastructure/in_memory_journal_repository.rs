use crate::domain::Journal;

use std::sync::{Arc, Mutex};

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

    pub fn save(&self, journal: Journal) {
        self.journals.lock().unwrap().push(journal);
    }

    pub fn all(&self) -> Vec<Journal> {
        self.journals.lock().unwrap().clone()
    }

    pub fn find(&self, id: uuid::Uuid) -> Option<Journal> {
        self.journals
            .lock()
            .unwrap()
            .iter()
            .find(|j| j.id == id)
            .cloned()
    }
}
