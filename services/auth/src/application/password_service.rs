use argon2::{
    password_hash::{
        PasswordHasher,
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
