use crate::application::default_chart::default_chart;
use crate::domain::{Account, ChartOfAccounts};

#[derive(Clone)]
pub struct ChartOfAccountsService {
    chart: ChartOfAccounts,
}

impl ChartOfAccountsService {
    pub fn new() -> Self {
        Self {
            chart: default_chart(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.chart.add_account(account);
    }

    pub fn exists(&self, code: &str) -> bool {
        self.chart.exists(code)
    }

    pub fn get(&self, code: &str) -> Option<&Account> {
        self.chart.get(code)
    }

    pub fn total_accounts(&self) -> usize {
        self.chart.total_accounts()
    }

    pub fn all(&self) -> Vec<&Account> {
        self.chart.all()
    }
}
