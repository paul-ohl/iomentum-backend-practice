#![allow(unused)]

use iommentum_backend_practice::{models::tickets::TicketModel, routes, Cfg, Controller};
use sqlx::{postgres::PgPoolOptions, Executor};
use warp::Filter;

#[tokio::main]
async fn main() {
    let ticket_model = TicketModel::new(&Cfg::get().db_url).await;
    let controller = Controller::new(ticket_model);

    println!("Listening on port 3000");
    warp::serve(routes(controller))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
