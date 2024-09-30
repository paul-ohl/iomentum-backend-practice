use serde::{Deserialize, Serialize};

use crate::domain::{
    errors::Error,
    types::{PasswordHash, Role, Username},
};

#[derive(Deserialize, Debug)]
pub struct UserInputDto {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct NewUserDto {
    pub username: Username,
    pub password_hash: PasswordHash,
    pub role: Role,
}

impl TryFrom<UserInputDto> for NewUserDto {
    type Error = Error;
    fn try_from(value: UserInputDto) -> Result<Self, Self::Error> {
        Ok(Self {
            username: Username::new(&value.username)?,
            password_hash: PasswordHash::new(&value.password)?,
            role: Role::new(value.role)?,
        })
    }
}
