use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub role: String,
}

pub async fn get_all(db_pool: Arc<PgPool>) -> Vec<User> {
    let users: Vec<User> =
        sqlx::query_as("SELECT id, username, role, created_at, updated_at FROM users")
            .fetch_all(&*db_pool)
            .await
            .expect("Couldn't fetch users");
    users
}

pub async fn get_by_id(db_pool: Arc<PgPool>, id: Uuid) -> Option<User> {
    let user: Option<User> =
        sqlx::query_as("SELECT id, username, created_at, updated_at FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&*db_pool)
            .await
            .expect("Couldn't fetch user");
    user
}

pub async fn get_by_username(db_pool: Arc<PgPool>, username: String) -> Vec<User> {
    let user: Vec<User> = sqlx::query_as(
        "SELECT id, username, created_at, updated_at FROM users WHERE username = $1",
    )
    .bind(username)
    .fetch_all(&*db_pool)
    .await
    .expect("Couldn't fetch user");
    user
}

pub async fn create(db_pool: Arc<PgPool>, new_user: UserInput) -> Option<Uuid> {
    let res = sqlx::query!(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) returning id",
        new_user.username,
        new_user.password,
        new_user.role
    )
    .fetch_one(&*db_pool)
    .await
    .expect("Couldn't create user")
    .id;
    Some(res)
}

pub async fn update(db_pool: Arc<PgPool>, id: Uuid, new_user: UserInput) -> Option<Uuid> {
    let res = sqlx::query!("UPDATE users SET username = $1, password_hash = $2, updated_at = $3 WHERE id = $4 returning id",
            new_user.username,
            new_user.password,
            Utc::now(),
            id
        )
            .fetch_one(&*db_pool)
            .await
            .expect("Couldn't update user")
            .id;
    Some(res)
}

pub async fn delete_one(db_pool: Arc<PgPool>, id: Uuid) -> Option<Uuid> {
    let res = sqlx::query!("DELETE FROM users WHERE id = $1 returning id", id)
        .fetch_one(&*db_pool)
        .await
        .expect("Couldn't delete user")
        .id;
    Some(res)
}
