use crate::domain::Journal;
use crate::infrastructure::{
    in_memory_journal_repository::InMemoryJournalRepository, journal_repository::JournalRepository,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct JournalService {
    repository: InMemoryJournalRepository,
}

impl JournalService {
    pub fn new(repository: InMemoryJournalRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, journal: Journal) -> Result<Journal, String> {
        journal.validate()?;

        self.repository.save(journal.clone());

        Ok(journal)
    }

    pub fn list(&self) -> Vec<Journal> {
        self.repository.list()
    }

    pub fn find(&self, id: Uuid) -> Option<Journal> {
        self.repository.find_by_id(id)
    }
}
