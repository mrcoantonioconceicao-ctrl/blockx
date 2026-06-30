use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub user_id: String,
    pub currency: String,
    pub balance: i64,
    pub status: WalletStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletStatus {
    Active,
    Frozen,
    Closed,
}

impl Wallet {
    pub fn new(user_id: String, currency: String) -> Self {
        let now = Self::now();

        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            currency,
            balance: 0,
            status: WalletStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn credit(&mut self, amount: i64) {
        if amount <= 0 {
            return;
        }

        self.balance += amount;
        self.updated_at = Self::now();
    }

    pub fn debit(&mut self, amount: i64) -> Result<(), String> {
        if amount <= 0 {
            return Err("invalid amount".to_string());
        }

        if self.balance < amount {
            return Err("insufficient funds".to_string());
        }

        self.balance -= amount;
        self.updated_at = Self::now();

        Ok(())
    }

    pub fn freeze(&mut self) {
        self.status = WalletStatus::Frozen;
        self.updated_at = Self::now();
    }

    pub fn close(&mut self) {
        self.status = WalletStatus::Closed;
        self.updated_at = Self::now();
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
