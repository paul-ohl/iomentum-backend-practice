use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

mod tickets;
// mod users;
mod with_state;

use with_state::with_state;

use crate::AppState;

pub fn get_routes(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health().or(tickets::get_ticket_routes(app_state.clone()))
    // .or(users::get_user_routes(app_state))
}

/// A simple health-check route
fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health").and(warp::get()).map(|| "OK")
}
