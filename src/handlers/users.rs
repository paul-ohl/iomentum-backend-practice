use std::sync::Arc;

use sqlx::PgPool;
use warp::{reject::Rejection, reply::Reply};

use super::errors::result_to_warp_reply;
use crate::models::users::{self, UserInput};

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_users(db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let users = users::get_all(db_pool).await;
    result_to_warp_reply(users)
}

pub async fn get_user_by_id(id: uuid::Uuid, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let user = users::get_by_id(db_pool, id).await;
    result_to_warp_reply(user)
}

pub async fn get_user_by_username(username: String, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let users = users::get_by_username(db_pool, username).await;
    result_to_warp_reply(users)
}

pub async fn create_user(user_input: UserInput, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let user_id = users::create(db_pool, user_input).await;
    result_to_warp_reply(user_id)
}

pub async fn update_user(
    id: uuid::Uuid,
    user_input: UserInput,
    db_pool: Arc<PgPool>,
) -> ReplyRes<impl Reply> {
    let user_id = users::update(db_pool, id, user_input).await;
    result_to_warp_reply(user_id)
}

pub async fn delete_user(id: uuid::Uuid, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let user_id = users::delete_one(db_pool, id).await;
    result_to_warp_reply(user_id)
}
