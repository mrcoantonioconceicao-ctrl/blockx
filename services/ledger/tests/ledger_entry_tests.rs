use ledger::domain::ledger_entry::LedgerEntry;

use rust_decimal::Decimal;
use uuid::Uuid;

fn debit_entry(amount: i64) -> LedgerEntry {
    LedgerEntry::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        "BRL".to_string(),
        Decimal::from(amount),
        Decimal::ZERO,
    )
}

fn credit_entry(amount: i64) -> LedgerEntry {
    LedgerEntry::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        "BRL".to_string(),
        Decimal::ZERO,
        Decimal::from(amount),
    )
}

#[test]
fn should_create_debit_entry() {
    let entry = debit_entry(100);

    assert!(entry.is_debit());
    assert!(!entry.is_credit());
}

#[test]
fn should_create_credit_entry() {
    let entry = credit_entry(100);

    assert!(entry.is_credit());
    assert!(!entry.is_debit());
}

#[test]
fn should_return_debit_amount() {
    let entry = debit_entry(250);

    assert_eq!(entry.amount(), Decimal::from(250));
}

#[test]
fn should_return_credit_amount() {
    let entry = credit_entry(800);

    assert_eq!(entry.amount(), Decimal::from(800));
}

#[test]
fn should_store_currency() {
    let entry = debit_entry(10);

    assert_eq!(entry.currency, "BRL");
}

#[test]
fn should_have_unique_id() {
    let a = debit_entry(10);
    let b = debit_entry(10);

    assert_ne!(a.id, b.id);
}

#[test]
fn should_have_journal_id() {
    let journal = Uuid::new_v4();

    let entry = LedgerEntry::new(
        journal,
        Uuid::new_v4(),
        "BRL".into(),
        Decimal::from(50),
        Decimal::ZERO,
    );

    assert_eq!(entry.journal_id, journal);
}

#[test]
fn should_have_account_id() {
    let account = Uuid::new_v4();

    let entry = LedgerEntry::new(
        Uuid::new_v4(),
        account,
        "BRL".into(),
        Decimal::from(50),
        Decimal::ZERO,
    );

    assert_eq!(entry.account_id, account);
}
