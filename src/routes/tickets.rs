use std::sync::Arc;

use crate::{handlers, AppState};

use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};

use super::with_state;

pub fn get_ticket_routes(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_all_tickets(app_state.clone())
        .or(get_by_id(app_state.clone()))
        .or(get_by_username(app_state.clone()))
        .or(create_ticket(app_state.clone()))
        .or(update_ticket(app_state.clone()))
        .or(delete_ticket(app_state))
}

fn get_all_tickets(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::tickets::get_all_tickets)
}

fn get_by_id(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::tickets::get_ticket_by_id)
}

fn get_by_username(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / "by-user" / Uuid)
        .and(warp::get())
        .and(with_state(app_state))
        .and_then(handlers::tickets::get_ticket_by_user_id)
}

fn create_ticket(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::tickets::create_ticket)
}

fn update_ticket(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_state(app_state))
        .and_then(handlers::tickets::update_ticket)
}

fn delete_ticket(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::delete())
        .and(with_state(app_state))
        .and_then(handlers::tickets::delete_ticket)
}
