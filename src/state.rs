use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::handlers::jwt_handler::JwtHandler;

pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_handler: JwtHandler,
}

pub async fn create_state(db_url: String, jwt_secret: String) -> Arc<AppState> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("cannot log to db");

    let jwt_handler = JwtHandler::new(jwt_secret).expect("cannot create jwt handler");

    Arc::new(AppState {
        db_pool,
        jwt_handler,
    })
}
