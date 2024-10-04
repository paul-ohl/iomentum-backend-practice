use std::sync::Arc;

use iomentum_backend_practice::{
    models::{pg_tickets::PgTicketsModel, pg_users::PgUsersModel},
    routes::get_routes,
    AppState, Cfg,
};

#[tokio::main]
async fn main() {
    let config = Cfg::init();

    let ticket_model = PgTicketsModel::new(config.db_url())
        .await
        .expect("Failed to create ticket model");
    let user_model = PgUsersModel::new(config.db_url())
        .await
        .expect("Failed to create user model");
    let app_state = AppState::new(
        config.jwt_secret,
        Box::new(user_model),
        Box::new(ticket_model),
    );
    let app_state = Arc::new(app_state);

    let routes = get_routes(app_state);

    println!("Listening on port 3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
