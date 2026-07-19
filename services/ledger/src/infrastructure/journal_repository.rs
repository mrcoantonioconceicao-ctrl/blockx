use crate::domain::Journal;

pub trait JournalRepository: Send + Sync {
    fn save(&self, journal: Journal);

    fn find_by_id(&self, id: &str) -> Option<Journal>;

    fn list(&self) -> Vec<Journal>;
}
