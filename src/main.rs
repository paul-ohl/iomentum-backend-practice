use iomentum_backend_practice::{routes::get_routes, state::create_state, Cfg};

#[tokio::main]
async fn main() {
    let config = Cfg::init();
    let app_state = create_state(config.db_url(), config.jwt_secret).await;

    let routes = get_routes(app_state);

    println!("Listening on port 3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
