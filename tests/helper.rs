use std::net::TcpListener;
use std::sync::Arc;

use iomentum_backend_practice::handlers::jwt_handler::JwtHandler;
use iomentum_backend_practice::routes::get_routes;
use iomentum_backend_practice::{AppState, Cfg};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub app_state: Arc<AppState>,
}

pub async fn spawn_app() -> TestApp {
    let available_port;
    {
        let listener = TcpListener::bind("0.0.0.0:0").unwrap();
        available_port = listener.local_addr().unwrap().port();
    }

    let mut config = Cfg::init();
    config.db_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&config).await;
    let jwt_handler = JwtHandler::new(config.jwt_secret).expect("cannot create jwt handler");
    let app_state = Arc::new(iomentum_backend_practice::AppState {
        db_pool,
        jwt_handler,
    });

    let routes = get_routes(app_state.clone());

    tokio::spawn(async move {
        warp::serve(routes)
            .run(([127, 0, 0, 1], available_port))
            .await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    TestApp {
        address: format!("http://127.0.0.1:{}", available_port),
        app_state,
    }
}

async fn configure_database(config: &Cfg) -> PgPool {
    let connection_options = PgConnectOptions::new()
        .host("localhost")
        .port(config.db_port.parse().unwrap())
        .username(&config.db_user)
        .password(&config.db_password);

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Could not connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create the database");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url())
        .await
        .expect("cannot log to db");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the db");
    db_pool
}
