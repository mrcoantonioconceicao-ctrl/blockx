use argon2::{
    password_hash::{
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string()
}

pub fn verify_password(
    password: &str,
    hash: &str,
) -> bool {
    let parsed_hash = PasswordHash::new(hash);

    match parsed_hash {
        Ok(hash) => {
            Argon2::default()
                .verify_password(
                    password.as_bytes(),
                    &hash,
                )
                .is_ok()
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_verify_correct_password() {
        let password = "123456";

        let hash =
            hash_password(password);

        assert!(
            verify_password(
                password,
                &hash
            )
        );
    }

    #[test]
    fn should_reject_invalid_password() {
        let password = "123456";

        let hash =
            hash_password(password);

        assert!(
            !verify_password(
                "wrong-password",
                &hash
            )
        );
    }
}
