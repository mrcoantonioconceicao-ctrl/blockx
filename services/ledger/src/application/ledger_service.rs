use crate::domain::ledger_entry::LedgerEntry;
use crate::infrastructure::ledger_repository::LedgerRepository;

/// Serviço responsável pelos casos de uso do Ledger.
///
/// Toda movimentação financeira deve passar por esta camada.
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

    /// Registra um novo lançamento contábil.
    pub fn record_entry(
        &self,
        debit_account: String,
        credit_account: String,
        amount: f64,
        currency: String,
        description: String,
    ) -> LedgerEntry {
        let entry = LedgerEntry::new(
            debit_account,
            credit_account,
            amount,
            currency,
            description,
        );

        self.repository.save(entry.clone());

        entry
    }

    /// Retorna todos os lançamentos.
    pub fn all_entries(&self) -> Vec<LedgerEntry> {
        self.repository.find_all()
    }

    /// Retorna o histórico de uma conta.
    pub fn account_history(
        &self,
        account: &str,
    ) -> Vec<LedgerEntry> {
        self.repository.find_by_account(account)
    }
}
