use crate::domain::journal::Journal;

pub struct JournalValidator;

impl JournalValidator {
    pub fn validate(journal: &Journal) -> Result<(), String> {
        journal.validate()?;

        let first_currency = journal
            .entries
            .first()
            .ok_or_else(|| "Journal sem lançamentos.".to_string())?
            .currency()
            .clone();

        for entry in &journal.entries {
            if entry.is_negative() {
                return Err(format!(
                    "Valor negativo encontrado para a conta {}.",
                    entry.account_id
                ));
            }

            if entry.currency() != &first_currency {
                return Err("Todas as entradas do Journal devem possuir a mesma moeda.".into());
            }
        }

        Ok(())
    }
}
