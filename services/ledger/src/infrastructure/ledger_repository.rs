use crate::domain::ledger_entry::LedgerEntry;
use uuid::Uuid;

/// Contrato do repositório do Ledger.
///
/// O Ledger é a fonte oficial da verdade (Source of Truth).
/// Todas as movimentações financeiras devem ser persistidas aqui.
pub trait LedgerRepository: Send + Sync {

    /// Persiste um novo lançamento.
    fn save(&self, entry: LedgerEntry);

    /// Busca um lançamento pelo seu ID.
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> Option<LedgerEntry>;

    /// Retorna todos os lançamentos.
    fn find_all(&self) -> Vec<LedgerEntry>;

    /// Retorna todos os lançamentos de uma conta.
    fn find_by_account(
        &self,
        account: &str,
    ) -> Vec<LedgerEntry>;
}
