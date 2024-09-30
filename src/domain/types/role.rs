use serde::Serialize;

use crate::domain::errors::{Error, Result};

const AVAILABLE_ROLES: [&str; 2] = ["user", "admin"];

#[derive(Debug, Serialize)]
pub struct Role(String);

impl Role {
    pub fn new(r: String) -> Result<Self> {
        let role = r.to_lowercase();
        if !AVAILABLE_ROLES.iter().any(|&e| e == role) {
            return Err(Error::InvalidRole(role));
        }
        Ok(Self(role))
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_new() {
        let role = "user";
        let role = Role::new(role.to_string()).unwrap();
        assert_eq!(role.as_ref(), "user");

        let role = "admin";
        let role = Role::new(role.to_string()).unwrap();
        assert_eq!(role.as_ref(), "admin");

        let role = "USER";
        let role = Role::new(role.to_string()).unwrap();
        assert_eq!(role.as_ref(), "user");
    }

    #[test]
    fn test_role_new_invalid_role() {
        let role = "invalid";
        let role = Role::new(role.to_string());
        assert!(role.is_err());
    }
}
