use std::sync::Arc;

use crate::handlers;

use sqlx::PgPool;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};

use super::with_db;

pub fn get_user_routes(
    db: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_all_users(db.clone())
        .or(get_by_id(db.clone()))
        .or(get_by_username(db.clone()))
        .or(create_user(db.clone()))
        .or(update_user(db.clone()))
        .or(delete_user(db))
}

fn get_all_users(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::users::get_all_users)
}

fn get_by_id(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::users::get_user_by_id)
}

fn get_by_username(
    db: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / "by-username" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::users::get_user_by_username)
}

fn create_user(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::users::create_user)
}

fn update_user(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::users::update_user)
}

fn delete_user(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::users::delete_user)
}
