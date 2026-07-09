use std::fmt;

#[derive(Debug, Clone)]
pub enum LedgerError {
    AccountNotFound(String),
    AccountInactive(String),
    InvalidCurrency(String),
    UnbalancedJournal,
    EmptyJournal,
    RepositoryError(String),
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::AccountNotFound(code) => {
                write!(f, "Account '{}' not found.", code)
            }
            LedgerError::AccountInactive(code) => {
                write!(f, "Account '{}' is inactive.", code)
            }
            LedgerError::InvalidCurrency(currency) => {
                write!(f, "Invalid currency '{}'.", currency)
            }
            LedgerError::UnbalancedJournal => {
                write!(f, "Journal is not balanced.")
            }
            LedgerError::EmptyJournal => {
                write!(f, "Journal contains no entries.")
            }
            LedgerError::RepositoryError(message) => {
                write!(f, "Repository error: {}", message)
            }
        }
    }
}

impl std::error::Error for LedgerError {}

