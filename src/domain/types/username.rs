use serde::Serialize;

use crate::domain::errors::{Error, Result};

#[derive(Debug, Serialize)]
pub struct Username(String);

impl Username {
    pub fn new(u: &str) -> Result<Self> {
        let s = u.trim().to_string();
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if s.is_empty() {
            Err(Error::InvalidUsername(
                "Username cannot be empty".to_string(),
            ))
        } else if s.chars().any(|g| forbidden_characters.contains(&g)) {
            Err(Error::InvalidUsername(
                "Username cannot contain special characters".to_string(),
            ))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_new() {
        let username = "username";
        let username = Username::new(username).unwrap();
        assert_eq!(username.as_ref(), "username");

        let username = " us$rname ";
        let username = Username::new(username).unwrap();
        assert_eq!(username.as_ref(), "us$rname");
    }

    #[test]
    fn test_username_new_invalid_username() {
        let username = "";
        let username = Username::new(username);
        assert!(username.is_err());

        let username = "us/ername";
        let username = Username::new(username);
        assert!(username.is_err());
    }
}
