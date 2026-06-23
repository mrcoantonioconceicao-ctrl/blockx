use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,          // subject (user id)
    pub exp: usize,           // expiration timestamp
    pub token_type: String,   // "access" ou "refresh"
}

pub struct RefreshTokenService {
    secret: String,
}

impl RefreshTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("Erro ao calcular expiração")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            token_type: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .expect("Erro ao gerar access token")
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("Erro ao calcular expiração")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            token_type: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .expect("Erro ao gerar refresh token")
    }

    pub fn validate_token(&self, token: &str) -> Option<Claims> {
        let validation = Validation::default();
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        ) {
            Ok(token_data) => Some(token_data.claims),
            Err(_) => None,
        }
    }

    pub fn renew_access_token(&self, refresh_token: &str) -> Option<String> {
        if let Some(claims) = self.validate_token(refresh_token) {
            if claims.token_type == "refresh" {
                let user_id = Uuid::parse_str(&claims.sub).ok()?;
                return Some(self.generate_access_token(user_id));
            }
        }
        None
    }
}
