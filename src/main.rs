use std::sync::Arc;

use iommentum_backend_practice::{routes::get_routes, Cfg};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let config = Cfg::init();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .expect("cannot log to db");
    let db_pool = Arc::new(db_pool);

    let routes = get_routes(db_pool);

    println!("Listening on port 3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
