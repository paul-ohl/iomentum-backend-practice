use crate::{
    domain::errors::{Error, Result},
    handlers::password_hasher::hash_password,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Create a new PasswordHash from a cleartext password.
    ///
    /// The function will validate and hash the password.
    pub fn new(password: String) -> Result<Self> {
        if password.len() < 8 {
            return Err(Error::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }

        let hash = hash_password(&password)
            .map_err(|_| Error::InvalidPassword("Failed to hash password".to_string()))?;
        Ok(Self(hash))
    }

    pub fn into_string(&self) -> &str {
        &self.0
    }
}
