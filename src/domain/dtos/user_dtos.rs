use serde::{Deserialize, Serialize};

use crate::domain::{
    errors::Error,
    types::{
        user_types::{NewUser, User},
        PasswordHash, Role, Username,
    },
};

#[derive(Deserialize, Debug)]
pub struct NewUserDto {
    pub username: String,
    pub password: String,
    pub role: String,
}

impl TryFrom<NewUserDto> for NewUser {
    type Error = Error;
    fn try_from(value: NewUserDto) -> Result<Self, Self::Error> {
        Ok(Self {
            username: Username::new(&value.username)?,
            password_hash: PasswordHash::new(&value.password)?,
            role: Role::new(value.role)?,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct UserLoginInputDto {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserDto {
    pub id: uuid::Uuid,
    pub username: String,
    pub role: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username.as_ref().to_string(),
            role: user.role.to_string(),
        }
    }
}
