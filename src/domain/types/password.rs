use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

use crate::{
    domain::errors::{Error, Result},
    handlers::password_hasher::hash_password,
};

#[derive(Debug, Clone)]
pub struct PasswordHash(SecretString);

impl PasswordHash {
    /// Create a new PasswordHash from a cleartext password.
    ///
    /// The function will validate and hash the password.
    pub fn new(password: &str) -> Result<Self> {
        if password.len() < 8 {
            return Err(Error::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }

        let hash = hash_password(password)
            .map_err(|_| Error::InvalidPassword("Failed to hash password".to_string()))?;
        Ok(Self(hash.into()))
    }

    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl Serialize for PasswordHash {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("[secret]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_new() {
        let password = "password";
        let password_hash = PasswordHash::new(password).unwrap();
        assert_ne!(password, password_hash.expose_secret());
    }

    #[test]
    fn test_password_hash_new_invalid_password() {
        let password = "pass";
        let password_hash = PasswordHash::new(password);
        assert!(password_hash.is_err());
    }

    #[test]
    fn test_password_serialization_displays_nothing() {
        let password = "password";
        let password_hash = PasswordHash::new(password).unwrap();

        let serialized = serde_json::to_string(&password_hash).unwrap();
        assert_eq!(serialized, "\"[secret]\"");
    }
}
