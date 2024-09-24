use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::Controller;

pub fn routes(
    controller: Controller,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let controller = Arc::new(controller);
    let health = warp::path!("health").map(Controller::health);

    let get_all_tickets = warp::path!("tickets").and_then(move || {
        let controller_clone = controller.clone();
        async move { controller_clone.get_all_tickets().await }
    });

    health.or(get_all_tickets)
}
