use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::domain::{
    errors::{Error, Result},
    types::JwtClaims,
};

pub struct JwtHandler {
    key: Hmac<Sha256>,
}

impl JwtHandler {
    pub fn new(secret: String) -> Result<Self> {
        Ok(Self {
            key: Hmac::new_from_slice(secret.as_bytes())
                .expect("Failed to create HMAC from secret:"),
        })
    }

    pub fn generate_token(&self, claims: JwtClaims) -> Result<String> {
        claims
            .to_tree_map()
            .sign_with_key(&self.key)
            .map_err(|e| Error::InternalError(format!("Failed to sign token: {}", e)))
    }

    pub fn decode_token(&self, token: &str) -> Result<JwtClaims> {
        let tree: BTreeMap<String, String> = token
            .verify_with_key(&self.key)
            .map_err(|e| Error::LoginFailed(format!("Couldn't decode JWT: {e}")))?;
        tree.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_generate_token() {
        let jwt_handler = JwtHandler::new("secret".to_string()).unwrap();
        let claims = JwtClaims::new("username".to_string(), Uuid::new_v4(), "role".to_string());
        let token = jwt_handler.generate_token(claims).unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_decode_token() {
        let jwt_handler = JwtHandler::new("secret".to_string()).unwrap();
        let claims = JwtClaims::new("username".to_string(), Uuid::new_v4(), "role".to_string());
        let token = jwt_handler.generate_token(claims).unwrap();
        let decoded_claims = jwt_handler.decode_token(&token).unwrap();
        assert_eq!(decoded_claims.sub, "username");
        assert_eq!(decoded_claims.role, "role");
        assert!(decoded_claims.exp > Utc::now().timestamp());
    }
}
