use rust_decimal::Decimal;
use uuid::Uuid;

use ledger::domain::journal::{Journal, JournalEntry};

fn entry(debit: i64, credit: i64) -> JournalEntry {
    JournalEntry {
        account_id: Uuid::new_v4(),
        currency: "BRL".to_string(),
        debit: Decimal::from(debit),
        credit: Decimal::from(credit),
    }
}

#[test]
fn should_create_balanced_journal() {
    let journal = Journal::new(vec![entry(100, 0), entry(0, 100)]);

    assert!(journal.is_balanced());
}

#[test]
fn should_reject_unbalanced_journal() {
    let journal = Journal::new(vec![entry(100, 0), entry(0, 90)]);

    assert!(!journal.is_balanced());
}

#[test]
fn should_calculate_total_debit() {
    let journal = Journal::new(vec![entry(100, 0), entry(50, 0), entry(0, 150)]);

    assert_eq!(journal.total_debit(), Decimal::from(150));
}

#[test]
fn should_calculate_total_credit() {
    let journal = Journal::new(vec![entry(100, 0), entry(0, 40), entry(0, 60)]);

    assert_eq!(journal.total_credit(), Decimal::from(100));
}

#[test]
fn should_validate_balanced_journal() {
    let journal = Journal::new(vec![entry(100, 0), entry(0, 100)]);

    assert!(journal.validate().is_ok());
}

#[test]
fn should_fail_empty_journal() {
    let journal = Journal::new(vec![]);

    assert!(journal.validate().is_err());
}

#[test]
fn should_fail_unbalanced_validation() {
    let journal = Journal::new(vec![entry(100, 0), entry(0, 50)]);

    assert!(journal.validate().is_err());
}
