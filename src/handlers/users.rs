use std::{convert::Infallible, sync::Arc};

use serde_json::json;
use sqlx::PgPool;
use warp::reply;

use crate::models::users;

pub async fn get_all_users(db_pool: Arc<PgPool>) -> Result<impl warp::Reply, Infallible> {
    let users = users::get_all(db_pool).await;
    Ok(warp::reply::json(&users))
}

pub async fn get_user_by_id(
    id: uuid::Uuid,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let user = users::get_by_id(db_pool, id).await;
    match user {
        Some(user) => Ok(reply::with_status(
            reply::json(&user),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"user not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}

pub async fn get_user_by_username(
    username: String,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let users = users::get_by_username(db_pool, username).await;
    Ok(warp::reply::json(&users))
}

pub async fn create_user(
    user_input: crate::models::users::UserInput,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let user_id = users::create(db_pool, user_input).await;
    Ok(reply::json(&json!({
        "id": user_id
    })))
}

pub async fn update_user(
    id: uuid::Uuid,
    user_input: crate::models::users::UserInput,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let user_id = users::update(db_pool, id, user_input).await;
    match user_id {
        Some(user) => Ok(reply::with_status(
            reply::json(&json!({
                "id": user
            })),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"user not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}

pub async fn delete_user(
    id: uuid::Uuid,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let user_id = users::delete_one(db_pool, id).await;
    match user_id {
        Some(user_id) => Ok(reply::with_status(
            reply::json(&user_id),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"user not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}
