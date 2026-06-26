use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::jwt_claims::Claims;

const SECRET: &[u8] = b"blockx_secret_key_change_me";

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let iat = now();
    let exp = iat + 60 * 60; // 1 hora

    let claims = Claims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| "token generation failed".to_string())
}

pub fn validate_token(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| "invalid token".to_string())?;

    Ok(token_data.claims)
}
