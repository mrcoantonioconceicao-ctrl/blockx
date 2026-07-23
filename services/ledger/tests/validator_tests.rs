use ledger::application::journal_validator::JournalValidator;
use ledger::domain::journal::{Journal, JournalEntry};

use rust_decimal::Decimal;
use uuid::Uuid;

fn entry(currency: &str, debit: i64, credit: i64) -> JournalEntry {
    JournalEntry {
        account_id: Uuid::new_v4(),
        currency: currency.to_string(),
        debit: Decimal::from(debit),
        credit: Decimal::from(credit),
    }
}

#[test]
fn should_accept_valid_journal() {
    let journal = Journal::new(vec![entry("BRL", 100, 0), entry("BRL", 0, 100)]);

    assert!(JournalValidator::validate(&journal).is_ok());
}

#[test]
fn should_reject_empty_journal() {
    let journal = Journal::new(vec![]);

    assert!(JournalValidator::validate(&journal).is_err());
}

#[test]
fn should_reject_unbalanced_journal() {
    let journal = Journal::new(vec![entry("BRL", 100, 0), entry("BRL", 0, 90)]);

    assert!(JournalValidator::validate(&journal).is_err());
}

#[test]
fn should_reject_negative_debit() {
    let journal = Journal::new(vec![entry("BRL", -100, 0), entry("BRL", 0, -100)]);

    assert!(JournalValidator::validate(&journal).is_err());
}

#[test]
fn should_reject_mixed_currency() {
    let journal = Journal::new(vec![entry("BRL", 100, 0), entry("USD", 0, 100)]);

    assert!(JournalValidator::validate(&journal).is_err());
}

#[test]
fn should_accept_large_values() {
    let journal = Journal::new(vec![entry("BRL", 1_000_000, 0), entry("BRL", 0, 1_000_000)]);

    assert!(JournalValidator::validate(&journal).is_ok());
}

#[test]
fn should_accept_zero_balance() {
    let journal = Journal::new(vec![entry("BRL", 0, 0)]);

    assert!(JournalValidator::validate(&journal).is_ok());
}

#[test]
fn should_accept_multiple_entries() {
    let journal = Journal::new(vec![
        entry("BRL", 50, 0),
        entry("BRL", 50, 0),
        entry("BRL", 0, 100),
    ]);

    assert!(JournalValidator::validate(&journal).is_ok());
}
