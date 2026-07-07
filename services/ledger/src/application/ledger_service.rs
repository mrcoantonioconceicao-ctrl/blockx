use crate::domain::{Journal, LedgerEntry};
use crate::infrastructure::ledger_repository::LedgerRepository;

#[derive(Clone)]
pub struct LedgerService<R>
where
    R: LedgerRepository + Clone,
{
    repository: R,
}

impl<R> LedgerService<R>
where
    R: LedgerRepository + Clone,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn post_journal(
        &self,
        journal: Journal,
    ) -> Result<(), String> {
        if !journal.is_balanced() {
            return Err(
                "Journal is not balanced (debits must equal credits)"
                    .to_string(),
            );
        }

        for entry in journal.entries {
            self.repository.save(entry);
        }

        Ok(())
    }

    pub fn list_entries(&self) -> Vec<LedgerEntry> {
        self.repository.find_all()
    }

    pub fn account_history(
        &self,
        account_id: String,
    ) -> Vec<LedgerEntry> {
        self.repository.find_by_account(&account_id)
    }
}

