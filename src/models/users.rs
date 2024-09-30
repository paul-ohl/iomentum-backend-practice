use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::domain::{
    dtos::user_dtos::NewUserDto,
    errors::{Error, Result},
};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub role: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_all(db_pool: Arc<PgPool>) -> Result<Vec<UserResponse>> {
    let users: Vec<UserResponse> =
        sqlx::query_as("SELECT id, username, role, created_at, updated_at FROM users")
            .fetch_all(&*db_pool)
            .await
            .map_err(Error::UserFetchFailed)?;
    Ok(users)
}

pub async fn get_by_id(db_pool: Arc<PgPool>, id: Uuid) -> Result<UserResponse> {
    let user: Option<UserResponse> = sqlx::query_as(
        "SELECT id, username, role, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&*db_pool)
    .await
    .map_err(Error::UserFetchFailed)?;
    match user {
        Some(user) => Ok(user),
        None => Err(Error::UserNotFound),
    }
}

pub async fn get_by_username(db_pool: Arc<PgPool>, username: String) -> Result<UserResponse> {
    let user: Option<UserResponse> = sqlx::query_as(
        "SELECT id, username, role, created_at, updated_at FROM users WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(&*db_pool)
    .await
    .map_err(Error::UserFetchFailed)?;
    match user {
        Some(user) => Ok(user),
        None => Err(Error::UserNotFound),
    }
}

pub async fn create(db_pool: Arc<PgPool>, new_user: NewUserDto) -> Result<Value> {
    let res = sqlx::query!(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) returning id",
        new_user.username.as_ref(),
        new_user.password_hash.expose_secret(),
        new_user.role.as_ref(),
    )
    .fetch_one(&*db_pool)
    .await
    .map_err(Error::UserCreationFailed)?
    .id;
    Ok(json!({ "id": res }))
}

pub async fn update(db_pool: Arc<PgPool>, id: Uuid, new_user: NewUserDto) -> Result<Value> {
    let res = sqlx::query!("UPDATE users SET username = $1, password_hash = $2, role = $3, updated_at = $4 WHERE id = $5 returning id",
            new_user.username.as_ref(),
            new_user.password_hash.expose_secret(),
            new_user.role.as_ref(),
            Utc::now(),
            id
        )
            .fetch_one(&*db_pool)
            .await
            .map_err(Error::UserUpdateFailed)?
            .id;
    Ok(json!({ "id": res }))
}

pub async fn delete(db_pool: Arc<PgPool>, id: Uuid) -> Result<Value> {
    let res = sqlx::query!("DELETE FROM users WHERE id = $1 returning id", id)
        .fetch_one(&*db_pool)
        .await
        .map_err(Error::UserDeletionFailed)?
        .id;
    Ok(json!({ "id": res }))
}
