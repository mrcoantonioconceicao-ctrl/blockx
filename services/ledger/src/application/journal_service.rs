use crate::domain::Journal;
use crate::infrastructure::journal_repository::JournalRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct JournalService<R>
where
    R: JournalRepository + Clone,
{
    repository: R,
}

impl<R> JournalService<R>
where
    R: JournalRepository + Clone,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create(&self, journal: Journal) -> Result<(), String> {
        journal.validate()?;
        self.repository.save(journal);
        Ok(())
    }

    pub fn list(&self) -> Vec<Journal> {
        self.repository.list()
    }

    pub fn find(&self, id: Uuid) -> Option<Journal> {
        self.repository.find_by_id(id)
    }
}
