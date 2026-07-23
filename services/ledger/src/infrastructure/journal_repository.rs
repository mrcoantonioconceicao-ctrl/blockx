use crate::domain::Journal;
use uuid::Uuid;

/// Contrato do repositório de Journals.
///
/// A camada de aplicação conhece apenas esta trait.
/// A implementação concreta (memória, PostgreSQL, etc.)
/// fica isolada na infraestrutura.
pub trait JournalRepository: Send + Sync {
    /// Persiste um novo Journal.
    fn save(&self, journal: Journal);

    /// Busca um Journal pelo seu identificador.
    fn find_by_id(&self, id: Uuid) -> Option<Journal>;

    /// Lista todos os Journals.
    fn list(&self) -> Vec<Journal>;
}
