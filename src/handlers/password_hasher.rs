use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher as ArgonPasswordHasher,
    PasswordVerifier,
};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| {
            "Failed to hash password, please try again or contact support if the problem persists"
                .to_string()
        })?
        .to_string();
    Ok(password_hash)
}

pub fn verify(password_sent: &str, db_hash: &str) -> bool {
    PasswordHash::new(db_hash).map_or(false, |parsed_hash| {
        Argon2::default()
            .verify_password(password_sent.as_bytes(), &parsed_hash)
            .is_ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password";
        let hashed_password = hash_password(password).unwrap();
        assert_ne!(password, hashed_password);
    }

    #[test]
    fn test_verify() {
        let password = "password";
        let hashed_password = hash_password(password).unwrap();
        assert!(verify(password, &hashed_password));
    }
}
