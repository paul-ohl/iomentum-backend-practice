mod helper;
use helper::spawn_app;
use iomentum_backend_practice::handlers::password_hasher;
use serde_json::json;

#[tokio::test]
async fn crud_users_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

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
    let saved = sqlx::query!("SELECT id, username, password_hash, role FROM Users")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch from db.");
    assert_eq!("test1", saved.username);
    assert_eq!("user", saved.role);
    assert!(password_hasher::verify("test1234", &saved.password_hash));
    let id = saved.id;

    // get user by id
    let response = client
        .get(format!("{}/users/{}", test_app.address, id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(
        "test1",
        response.json::<serde_json::Value>().await.unwrap()["username"]
    );

    let response = client
        .post(format!("{}/register", test_app.address))
        .body(
            json!({
                "username": "test2",
                "password": "2short",
                "role": "user",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(!response.status().is_success());
    insta::assert_json_snapshot!(response.json::<serde_json::Value>().await.unwrap());

    // get all users
    let response = client
        .get(format!("{}/users", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(
        id.to_string(),
        response.json::<serde_json::Value>().await.unwrap()[0]["id"]
    );

    // get user by username
    let response = client
        .get(format!("{}/users/by-username/test1", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let data = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(id.to_string(), data["id"]);
    assert_eq!("test1", data["username"]);

    // update user
    let response = client
        .patch(format!("{}/users/{}", test_app.address, id))
        .body(
            json!({
                "username": "test3",
                "password": "newpassword",
                "role": "admin",
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let saved = sqlx::query!("SELECT id, username, password_hash, role FROM Users")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch from db.");
    assert_eq!("test3", saved.username);
    assert_eq!("admin", saved.role);
    assert!(password_hasher::verify("newpassword", &saved.password_hash));
}
