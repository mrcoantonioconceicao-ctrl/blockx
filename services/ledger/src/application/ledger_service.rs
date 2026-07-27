use crate::domain::journal::Journal;
use crate::domain::ledger_entry::LedgerEntry;
use crate::infrastructure::ledger_repository::LedgerRepository;

use std::collections::HashSet;
use uuid::Uuid;

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

    /// Publica um Journal no Livro Razão.
    ///
    /// Etapas:
    /// 1. Valida o Journal.
    /// 2. Garante que não existam contas duplicadas.
    /// 3. Converte cada JournalEntry em LedgerEntry.
    /// 4. Persiste no Ledger.
    pub fn post_journal(&self, journal: &Journal) -> Result<(), String> {
        journal.validate()?;

        self.validate_accounts(journal)?;

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

    fn validate_accounts(&self, journal: &Journal) -> Result<(), String> {
        let mut accounts = HashSet::<Uuid>::new();

        for entry in &journal.entries {
            if !accounts.insert(entry.account_id) {
                return Err(format!(
                    "Conta duplicada encontrada no Journal: {}",
                    entry.account_id
                ));
            }
        }

        Ok(())
    }
}
