use crate::domain::Journal;
use crate::infrastructure::in_memory_journal_repository::InMemoryJournalRepository;

#[derive(Clone)]
pub struct JournalService {
    repository: InMemoryJournalRepository,
}

impl JournalService {
    pub fn new(repository: InMemoryJournalRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, journal: Journal) -> Result<(), String> {
        journal.validate()?;
        self.repository.save(journal);
        Ok(())
    }

    pub fn list(&self) -> Vec<Journal> {
        self.repository.all()
    }

    pub fn find(&self, id: uuid::Uuid) -> Option<Journal> {
        self.repository.find(id)
    }
}
