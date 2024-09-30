mod helper;
use helper::spawn_app;
use serde_json::json;

#[tokio::test]
async fn create_users_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/users", test_app.address))
        .body(
            json!({
                "username": "test1",
                "password": "test1",
                "role": "user",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let saved = sqlx::query!("SELECT username, password_hash, role FROM Users")
        .fetch_one(&*test_app.db_pool)
        .await
        .expect("Failed to fetch from db.");
    assert_eq!("test1", saved.username);
    assert_eq!("user", saved.role);
    // assert!(
    //     test_app
    //         .app_state
    //         .hasher
    //         .verify(&saved.password_hash, "password"),
    //     "Current password hash is: {}",
    //     &saved.password_hash
    // );
}
