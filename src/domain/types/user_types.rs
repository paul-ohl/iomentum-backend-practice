use uuid::Uuid;

use crate::domain::types::{PasswordHash, Role, Username};

pub struct User {
    pub id: Uuid,
    pub username: Username,
    pub role: Role,
}

pub struct InternalUseUser {
    pub id: Uuid,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub role: Role,
}

pub struct NewUser {
    pub username: Username,
    pub password_hash: PasswordHash,
    pub role: Role,
}
