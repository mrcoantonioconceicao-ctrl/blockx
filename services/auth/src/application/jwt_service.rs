use config::AppConfig;

use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

use serde::{
    Deserialize,
    Serialize,
};

use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(
    user_id: &str,
) -> String {
    let config = AppConfig::load();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now
            + config.jwt_expiration_seconds,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            config.jwt_secret.as_bytes(),
        ),
    )
    .expect(
        "failed to generate jwt",
    )
}

pub fn validate_token(
    token: &str,
) -> Option<Claims> {
    let config = AppConfig::load();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            config.jwt_secret.as_bytes(),
        ),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_token() {
        let token =
            generate_token(
                "user-123",
            );

        assert!(
            !token.is_empty()
        );
    }

    #[test]
    fn should_validate_token() {
        let token =
            generate_token(
                "user-123",
            );

        let claims =
            validate_token(&token);

        assert!(
            claims.is_some()
        );

        assert_eq!(
            claims.unwrap().sub,
            "user-123"
        );
    }

    #[test]
    fn should_reject_invalid_token() {
        let claims =
            validate_token(
                "invalid-token",
            );

        assert!(
            claims.is_none()
        );
    }
}
