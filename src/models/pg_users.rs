use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::domain::{
    errors::{Error, Result},
    types::{
        user_types::{InternalUseUser, NewUser, User},
        PasswordHash, Role, Username,
    },
};

use super::users::UsersModel;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PgUser {
    pub id: Uuid,
    pub username: String,
    pub role: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PgUser> for User {
    fn from(user: PgUser) -> User {
        User {
            id: user.id,
            username: Username::new(&user.username).unwrap(),
            role: Role::new(user.role).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PgInternalUseUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

impl From<PgInternalUseUser> for InternalUseUser {
    fn from(user: PgInternalUseUser) -> InternalUseUser {
        InternalUseUser {
            id: user.id,
            username: Username::new(&user.username).unwrap(),
            password_hash: unsafe { PasswordHash::new_unchecked(user.password_hash) },
            role: Role::new(user.role).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserLoginVerification {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

pub struct PgUsersModel {
    db_pool: PgPool,
}

#[async_trait::async_trait]
impl UsersModel for PgUsersModel {
    async fn get_users(&self) -> Result<Vec<User>> {
        let users: Vec<PgUser> =
            sqlx::query_as("SELECT id, username, role, created_at, updated_at FROM users")
                .fetch_all(&self.db_pool)
                .await
                .map_err(Error::UserFetchFailed)?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    async fn get_user(&self, id: Uuid) -> Result<User> {
        let user: Option<PgUser> = sqlx::query_as(
            "SELECT id, username, role, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await
        .map_err(Error::UserFetchFailed)?;
        match user {
            Some(user) => Ok(user.into()),
            None => Err(Error::UserNotFound),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<User> {
        let user: Option<PgUser> = sqlx::query_as(
            "SELECT id, username, role, created_at, updated_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.db_pool)
        .await
        .map_err(Error::UserFetchFailed)?;
        match user {
            Some(user) => Ok(user.into()),
            None => Err(Error::UserNotFound),
        }
    }

    async fn get_user_by_username_interal(&self, username: String) -> Result<InternalUseUser> {
        let user: Option<PgInternalUseUser> = sqlx::query_as(
            "SELECT id, username, password_hash, role FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.db_pool)
        .await
        .map_err(Error::UserFetchFailed)?;
        match user {
            Some(user) => Ok(user.into()),
            None => Err(Error::UserNotFound),
        }
    }

    async fn create_user(&self, user: NewUser) -> Result<Uuid> {
        let res = sqlx::query!(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) returning id",
            user.username.as_ref(),
            user.password_hash.expose_secret(),
            user.role.as_ref(),
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(Error::UserCreationFailed)?
        .id;
        Ok(res)
    }

    async fn update_user(&self, id: Uuid, user: NewUser) -> Result<Uuid> {
        let res = sqlx::query!(
                "UPDATE users SET username = $1, password_hash = $2, role = $3, updated_at = $4 WHERE id = $5 returning id",
                user.username.as_ref(),
                user.password_hash.expose_secret(),
                user.role.as_ref(),
                Utc::now(),
                id
            )
            .fetch_one(&self.db_pool)
            .await
            .map_err(Error::UserUpdateFailed)?
            .id;
        Ok(res)
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.db_pool)
            .await
            .map_err(Error::UserDeletionFailed)?;
        Ok(())
    }
}

impl PgUsersModel {
    pub async fn new(db_url: String) -> std::result::Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Ok(Self { db_pool })
    }
}
