use std::sync::Arc;

use crate::handlers;

use sqlx::PgPool;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};

use super::with_db;

pub fn get_ticket_routes(
    db: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_all_tickets(db.clone())
        .or(get_by_id(db.clone()))
        .or(get_by_username(db.clone()))
        .or(create_ticket(db.clone()))
        .or(update_ticket(db.clone()))
        .or(delete_ticket(db))
}

fn get_all_tickets(
    db: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::tickets::get_all_tickets)
}

fn get_by_id(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::tickets::get_ticket_by_id)
}

fn get_by_username(
    db: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / "by-user" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::tickets::get_ticket_by_username)
}

fn create_ticket(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::tickets::create_ticket)
}

fn update_ticket(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::tickets::update_ticket)
}

fn delete_ticket(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tickets" / Uuid)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::tickets::delete_ticket)
}
