use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::LedgerEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn validate(&self) -> Result<(), String> {
        if self.entries.is_empty() {
            return Err("Journal must contain at least one ledger entry.".to_string());
        }

        for (index, entry) in self.entries.iter().enumerate() {
            if entry.debit_account.trim().is_empty() {
                return Err(format!(
                    "Entry {}: debit account cannot be empty.",
                    index + 1
                ));
            }

            if entry.credit_account.trim().is_empty() {
                return Err(format!(
                    "Entry {}: credit account cannot be empty.",
                    index + 1
                ));
            }

            if entry.debit_account == entry.credit_account {
                return Err(format!(
                    "Entry {}: debit and credit accounts must be different.",
                    index + 1
                ));
            }

            if entry.is_zero() {
                return Err(format!(
                    "Entry {}: amount must be greater than zero.",
                    index + 1
                ));
            }

            if entry.is_negative() {
                return Err(format!(
                    "Entry {}: amount cannot be negative.",
                    index + 1
                ));
            }
        }

        Ok(())
    }
}
