mod helper;

use helper::spawn_app;
use serde_json::json;

#[tokio::test]
async fn crud_users_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Register a user
    let response = client
        .post(format!("{}/register", test_app.address))
        .body(
            json!({
                "username": "test1",
                "password": "test1234",
                "role": "user",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());

    let response = client
        .post(format!("{}/login", test_app.address))
        .body(
            json!({
                "username": "test1",
                "password": "wrongpassword",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(!response.status().is_success());
    assert!(response.text().await.unwrap().contains("Wrong password"));

    let response = client
        .post(format!("{}/login", test_app.address))
        .body(
            json!({
                "username": "test1",
                "password": "test1234",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let jwt = response.text().await.unwrap();
    let jwt = jwt.trim_matches('"');
    let jwt_claims = test_app
        .app_state
        .jwt_handler
        .decode_token(jwt)
        .expect("Failed to decode token");
    assert_eq!("test1", jwt_claims.sub);
    assert_eq!("user", jwt_claims.role);
    assert!(jwt_claims.exp > chrono::Utc::now().timestamp());
    assert!(jwt_claims.iat <= chrono::Utc::now().timestamp());
}
