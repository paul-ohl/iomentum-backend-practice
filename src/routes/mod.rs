use std::sync::Arc;

use sqlx::PgPool;
use warp::{reject::Rejection, reply::Reply, Filter};

mod tickets;
mod users;
mod with_db;

use with_db::with_db;

pub fn get_routes(db: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health()
        .or(tickets::get_ticket_routes(db.clone()))
        .or(users::get_user_routes(db))
}

/// A simple health-check route
fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health").and(warp::get()).map(|| "OK")
}
