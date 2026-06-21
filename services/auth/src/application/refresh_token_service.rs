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

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RefreshClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_refresh_token(
    user_id: &str,
) -> String {

    let config = AppConfig::load();

    let now = SystemTime::now()
        .duration_since(
            UNIX_EPOCH,
        )
        .unwrap()
        .as_secs() as usize;

    let claims = RefreshClaims {

        sub: user_id.to_string(),

        iat: now,

        exp: now
            + config
                .refresh_token_expiration_seconds,
    };

    encode(
        &Header::default(),

        &claims,

        &EncodingKey::from_secret(
            config.jwt_secret.as_bytes(),
        ),
    )
    .expect(
        "failed to generate refresh token",
    )
}

pub fn validate_refresh_token(
    token: &str,
) -> Option<RefreshClaims> {

    let config = AppConfig::load();

    decode::<RefreshClaims>(
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
    fn should_generate_refresh_token() {

        let token =
            generate_refresh_token(
                "user-123",
            );

        assert!(
            !token.is_empty()
        );
    }

    #[test]
    fn should_validate_refresh_token() {

        let token =
            generate_refresh_token(
                "user-123",
            );

        let claims =
            validate_refresh_token(
                &token,
            );

        assert!(
            claims.is_some()
        );

        assert_eq!(
            claims.unwrap().sub,
            "user-123",
        );
    }

    #[test]
    fn should_reject_invalid_refresh_token() {

        let claims =
            validate_refresh_token(
                "invalid-token",
            );

        assert!(
            claims.is_none()
        );
    }
}
