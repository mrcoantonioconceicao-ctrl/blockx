use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::LedgerEntry;

#[derive(Debug, Clone)]
pub struct Journal {
    pub id: Uuid,
    pub description: String,
    pub entries: Vec<LedgerEntry>,
    pub created_at: DateTime<Utc>,
}

impl Journal {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            entries: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn add_entry(&mut self, entry: LedgerEntry) {
        self.entries.push(entry);
    }

    pub fn total_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn total_debits(&self) -> Decimal {
        self.entries
            .iter()
            .filter(|e| e.entry_type == "DEBIT")
            .map(|e| e.amount)
            .sum()
    }

    pub fn total_credits(&self) -> Decimal {
        self.entries
            .iter()
            .filter(|e| e.entry_type == "CREDIT")
            .map(|e| e.amount)
            .sum()
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debits() == self.total_credits()
    }
}
