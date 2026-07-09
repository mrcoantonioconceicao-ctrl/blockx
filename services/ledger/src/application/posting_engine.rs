use crate::application::errors::LedgerError;
use crate::application::journal_validator::JournalValidator;
use crate::domain::Journal;

pub struct PostingEngine;

impl PostingEngine {
    pub fn post(journal: &Journal) -> Result<(), LedgerError> {
        // Valida o Journal antes do processamento
        JournalValidator::validate(journal)?;

        // Próximas etapas da evolução:
        //
        // 1. Validar Chart of Accounts
        // 2. Validar moedas
        // 3. Abrir transação
        // 4. Persistir Journal
        // 5. Persistir Ledger Entries
        // 6. Commit/Rollback

        Ok(())
    }
}
