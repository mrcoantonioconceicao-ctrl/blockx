use crate::domain::wallet::Wallet;
use crate::infrastructure::wallet_repository::WalletRepository;

pub struct WalletService<R: WalletRepository> {
    repo: R,
}

impl<R: WalletRepository> WalletService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_wallet(&self, user_id: String, currency: String) -> Wallet {
        let wallet = Wallet::new(user_id, currency);
        self.repo.save(wallet.clone());
        wallet
    }

    pub fn get_wallet(&self, user_id: &str) -> Option<Wallet> {
        self.repo.find_by_user_id(user_id)
    }

    pub fn credit(&self, user_id: &str, amount: i64) -> Result<Wallet, String> {
        let mut wallet = self
            .repo
            .find_by_user_id(user_id)
            .ok_or("wallet not found")?;

        wallet.credit(amount);
        self.repo.update(wallet.clone());

        Ok(wallet)
    }

    pub fn debit(&self, user_id: &str, amount: i64) -> Result<Wallet, String> {
        let mut wallet = self
            .repo
            .find_by_user_id(user_id)
            .ok_or("wallet not found")?;

        wallet.debit(amount)?;
        self.repo.update(wallet.clone());

        Ok(wallet)
    }
}
