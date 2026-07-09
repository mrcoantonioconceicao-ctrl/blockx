use crate::domain::{Account, AccountType, ChartOfAccounts};

pub fn default_chart() -> ChartOfAccounts {
    let mut chart = ChartOfAccounts::new();

    chart.add_account(Account::new(
        "1001",
        "Cash",
        AccountType::Asset,
    ));

    chart.add_account(Account::new(
        "1100",
        "Bank",
        AccountType::Asset,
    ));

    chart.add_account(Account::new(
        "2000",
        "Accounts Payable",
        AccountType::Liability,
    ));

    chart.add_account(Account::new(
        "3000",
        "Equity",
        AccountType::Equity,
    ));

    chart.add_account(Account::new(
        "4000",
        "Revenue",
        AccountType::Revenue,
    ));

    chart.add_account(Account::new(
        "5000",
        "Expense",
        AccountType::Expense,
    ));

    chart
}
