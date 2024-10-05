mod helper;

use helper::spawn_app;
use iomentum_backend_practice::{domain::types::JwtClaims, handlers::jwt_handler::JwtHandler};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn login_works() {
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

#[tokio::test]
async fn accessing_protected_route_works() {
    let test_app = spawn_app().await;
    let _admin_user_id = helper::insert_user(&test_app, "admin_user", "admin").await;
    let _regular_user_id = helper::insert_user(&test_app, "regular_user", "user").await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/login", test_app.address))
        .body(
            json!({
                "username": "admin_user",
                "password": "test1234",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let admin_jwt = response.text().await.unwrap();
    let admin_jwt = admin_jwt.trim_matches('"');

    let response = client
        .get(format!("{}/tickets", test_app.address))
        .header("Authorization", format!("Bearer {}", admin_jwt))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());

    let response = client
        .post(format!("{}/login", test_app.address))
        .body(
            json!({
                "username": "regular_user",
                "password": "test1234",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let regular_jwt = response.text().await.unwrap();
    let regular_jwt = regular_jwt.trim_matches('"');

    let response = client
        .get(format!("{}/tickets", test_app.address))
        .header("Authorization", format!("Bearer {}", regular_jwt))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(!response.status().is_success());

    let jwt_handler = JwtHandler::new("incorrectsecret".to_string()).unwrap();
    let fake_claims = JwtClaims::new("test1".to_string(), Uuid::new_v4(), "admin".to_string());
    let fake_jwt = jwt_handler.generate_token(fake_claims).unwrap();
    let response = client
        .get(format!("{}/tickets", test_app.address))
        .header("Authorization", format!("Bearer {}", fake_jwt))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(!response.status().is_success());
}
