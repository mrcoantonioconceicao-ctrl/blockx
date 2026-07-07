use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::account_type::AccountType;

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

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
