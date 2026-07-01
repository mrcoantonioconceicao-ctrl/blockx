use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub active: bool,
}

impl Account {
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        account_type: AccountType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            code: code.into(),
            name: name.into(),
            account_type,
            active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
