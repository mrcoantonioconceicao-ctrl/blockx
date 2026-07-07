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
            .map(|entry| entry.amount.amount)
            .fold(Decimal::ZERO, |acc, value| acc + value)
    }

    pub fn total_credits(&self) -> Decimal {
        self.entries
            .iter()
            .map(|entry| entry.amount.amount)
            .fold(Decimal::ZERO, |acc, value| acc + value)
    }

    pub fn is_balanced(&self) -> bool {
        !self.entries.is_empty()
            && self.total_debits() == self.total_credits()
    }
}
