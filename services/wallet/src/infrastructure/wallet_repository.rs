use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::wallet::Wallet;

pub trait WalletRepository {
    fn save(&self, wallet: Wallet);
    fn find_by_user_id(&self, user_id: &str) -> Option<Wallet>;
    fn update(&self, wallet: Wallet);
}

pub struct InMemoryWalletRepository {
    wallets: Mutex<HashMap<String, Wallet>>,
}

impl InMemoryWalletRepository {
    pub fn new() -> Self {
        Self {
            wallets: Mutex::new(HashMap::new()),
        }
    }
}

impl WalletRepository for InMemoryWalletRepository {

    fn save(&self, wallet: Wallet) {
        let mut db = self.wallets.lock().unwrap();
        db.insert(wallet.user_id.clone(), wallet);
    }

    fn find_by_user_id(&self, user_id: &str) -> Option<Wallet> {
        let db = self.wallets.lock().unwrap();
        db.get(user_id).cloned()
    }

    fn update(&self, wallet: Wallet) {
        let mut db = self.wallets.lock().unwrap();
        db.insert(wallet.user_id.clone(), wallet);
    }
}
