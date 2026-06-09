use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

const JWT_SECRET: &[u8] =
    b"blockx-development-secret";

pub fn generate_token(
    user_id: &str,
) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            JWT_SECRET,
        ),
    )
    .expect(
        "failed to generate jwt",
    )
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
}
