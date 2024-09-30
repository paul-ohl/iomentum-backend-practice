use std::sync::Arc;

use sqlx::PgPool;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::dtos::user_dtos::UserInputDto, handlers::errors::result_to_warp_reply, models::users,
};

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

pub async fn update_user(
    id: uuid::Uuid,
    user_input: UserInputDto,
    db_pool: Arc<PgPool>,
) -> ReplyRes<impl Reply> {
    match user_input.try_into() {
        Ok(user_modifications) => {
            result_to_warp_reply(users::update(db_pool, id, user_modifications).await)
        }
        Err(e) => result_to_warp_reply(Err(e)),
    }
}

pub async fn delete_user(id: uuid::Uuid, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let user_id = users::delete(db_pool, id).await;
    result_to_warp_reply(user_id)
}

pub async fn register_user(user_input: UserInputDto, db_pool: Arc<PgPool>) -> ReplyRes<impl Reply> {
    match user_input.try_into() {
        Ok(new_user) => result_to_warp_reply(users::create(db_pool, new_user).await),
        Err(e) => result_to_warp_reply(Err(e)),
    }
}
