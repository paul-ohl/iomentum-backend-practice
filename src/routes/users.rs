use std::sync::Arc;

use crate::{handlers, AppState};

use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};

use super::with_state;

pub fn get_user_routes(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_all_users(app_state.clone())
        .or(get_by_id(app_state.clone()))
        .or(get_by_username(app_state.clone()))
        .or(create_user(app_state.clone())) // Should I delete this?
        .or(update_user(app_state.clone()))
        .or(register(app_state.clone()))
        .or(delete_user(app_state.clone()))
        .or(login(app_state))
}

fn get_all_users(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::users::get_all_users)
}

fn get_by_id(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::users::get_user_by_id)
}

fn get_by_username(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / "by-username" / String)
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::users::get_user_by_username)
}

fn create_user(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::users::register_user)
}

fn update_user(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::users::update_user)
}

fn delete_user(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users" / Uuid)
        .and(warp::delete())
        .and(with_state(app_state))
        .and_then(handlers::users::delete_user)
}

fn register(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::users::register_user)
}

fn login(app_state: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::users::login_user)
}
