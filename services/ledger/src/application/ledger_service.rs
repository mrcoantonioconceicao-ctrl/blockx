use crate::domain::journal::Journal;
use crate::domain::ledger_entry::LedgerEntry;
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

    pub fn post_journal(&self, journal: &Journal) -> Result<(), String> {
        journal.validate()?;

        for entry in &journal.entries {
            let ledger_entry = LedgerEntry::new(
                journal.id,
                entry.account_id,
                entry.currency.clone(),
                entry.debit,
                entry.credit,
            );

            self.repository.save(ledger_entry);
        }

        Ok(())
    }
}
