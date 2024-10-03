use std::net::TcpListener;
use std::sync::Arc;

use iomentum_backend_practice::handlers::password_hasher;
use iomentum_backend_practice::models::pg_tickets::PgTicketsModel;
use iomentum_backend_practice::routes::get_routes;
use iomentum_backend_practice::{AppState, Cfg};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub app_state: Arc<AppState>,
    pub db_pool: PgPool,
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

    let ticket_model = PgTicketsModel::new(config.db_url()).await.unwrap();
    let app_state = AppState::new(config.jwt_secret, ticket_model);
    let app_state = Arc::new(app_state);

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
        db_pool,
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

#[allow(dead_code)]
/// Insert a user into the database
/// Returns the id of the user
/// The password will always be "test1234"
/// The arguments must be valid data
pub async fn insert_user(test_app: &TestApp, username: &str, role: &str) -> Uuid {
    let password = "test1234";
    let hash = password_hasher::hash_password(password).unwrap();

    let user = sqlx::query!(
        r#"
        INSERT INTO Users (username, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        username,
        hash,
        role
    )
    .fetch_one(&test_app.db_pool)
    .await
    .unwrap();

    user.id
}
