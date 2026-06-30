use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa um lançamento contábil no Ledger.
///
/// Cada movimentação financeira deve gerar um ou mais lançamentos.
/// O Ledger é imutável: uma vez criado, um lançamento nunca deve ser alterado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Identificador único do lançamento.
    pub id: Uuid,

    /// Identificador da transação à qual este lançamento pertence.
    pub transaction_id: Uuid,

    /// Conta debitada.
    pub debit_account: String,

    /// Conta creditada.
    pub credit_account: String,

    /// Valor da movimentação.
    pub amount: f64,

    /// Moeda (BRL, BTC, USDT, ETH, etc.).
    pub currency: String,

    /// Descrição da operação.
    pub description: String,

    /// Data e hora do lançamento.
    pub created_at: DateTime<Utc>,
}

impl LedgerEntry {
    pub fn new(
        debit_account: String,
        credit_account: String,
        amount: f64,
        currency: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            transaction_id: Uuid::new_v4(),
            debit_account,
            credit_account,
            amount,
            currency,
            description,
            created_at: Utc::now(),
        }
    }
}
