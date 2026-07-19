use crate::{application::errors::LedgerError, domain::Journal};

pub struct JournalValidator;

impl JournalValidator {
    pub fn validate(journal: &Journal) -> Result<(), LedgerError> {
        if journal.entries.is_empty() {
            return Err(LedgerError::EmptyJournal);
        }

        if !journal.is_balanced() {
            return Err(LedgerError::UnbalancedJournal);
        }

        let first_currency = journal.entries[0].currency().clone();

        for entry in &journal.entries {
            if entry.is_negative() {
                return Err(LedgerError::RepositoryError(
                    "Negative amounts are not allowed.".into(),
                ));
            }

            if entry.currency() != &first_currency {
                return Err(LedgerError::InvalidCurrency(entry.currency().to_string()));
            }
        }

        Ok(())
    }
}
