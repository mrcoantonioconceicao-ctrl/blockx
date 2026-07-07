use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::{Currency, Money};
use uuid::Uuid;

/// Representa um lançamento contábil imutável do Ledger.
///
/// Todo lançamento pertence a uma transação e faz parte
/// de um Journal. Após criado, nunca deve ser alterado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Identificador único do lançamento.
    pub id: Uuid,

    /// Identificador da transação.
    pub transaction_id: Uuid,

    /// Conta debitada.
    pub debit_account: String,

    /// Conta creditada.
    pub credit_account: String,

    /// Valor monetário.
    pub amount: Money,

    /// Descrição do lançamento.
    pub description: String,

    /// Momento em que o lançamento foi criado.
    pub created_at: DateTime<Utc>,
}

impl LedgerEntry {
    pub fn new(
        debit_account: String,
        credit_account: String,
        amount: Decimal,
        currency: Currency,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            transaction_id: Uuid::new_v4(),
            debit_account,
            credit_account,
            amount: Money::new(amount, currency),
            description,
            created_at: Utc::now(),
        }
    }

    pub fn currency(&self) -> &Currency {
        &self.amount.currency
    }

    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.amount.is_negative()
    }
}
