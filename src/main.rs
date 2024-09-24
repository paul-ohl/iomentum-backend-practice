use std::sync::Arc;

use iommentum_backend_practice::Controller;
use warp::Filter;

#[tokio::main]
async fn main() {
    let controller = Arc::new(Controller::new());
    let health = warp::path!("health").map(move || controller.health());

    println!("Listening on port 3000");
    warp::serve(health).run(([127, 0, 0, 1], 3000)).await;
}
