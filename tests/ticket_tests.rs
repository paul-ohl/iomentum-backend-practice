mod helper;
use helper::{insert_user, spawn_app};
use serde_json::json;

#[tokio::test]
async fn crud_ticket_works() {
    let test_app = spawn_app().await;
    let test_user_id = insert_user(&test_app, "Test1", "user").await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/tickets", test_app.address))
        .body(
            json!({
                "owner_id": test_user_id, // This doesn't exist
                "concert_name": "Trivium",
                "concert_date": "2021-08-01T00:00:00Z",
                "barcode_data": "12345-abcde-67890",
                "price": 50.0,
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let saved = sqlx::query!(
        "SELECT id, owner_id, concert_name, concert_date, barcode_data, price FROM Tickets"
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch from db.");
    assert_eq!(test_user_id, saved.owner_id);
    assert_eq!("Trivium", saved.concert_name);
    assert_eq!("12345-abcde-67890", saved.barcode_data);
    assert_eq!(50.0, saved.price);
    let received_text = response.text().await.unwrap();
    assert_eq!(received_text.trim_matches('"'), saved.id.to_string());
    let id = saved.id;

    // get ticket by id
    let response = client
        .get(format!("{}/tickets/{}", test_app.address, id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(test_user_id, saved.owner_id);
    assert_eq!("Trivium", saved.concert_name);
    assert_eq!("12345-abcde-67890", saved.barcode_data);
    assert_eq!(50.0, saved.price);

    // get all tickets
    let response = client
        .get(format!("{}/tickets", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let data = response.json::<Vec<serde_json::Value>>().await.unwrap();
    assert_eq!(id.to_string(), data[0]["id"]);
    assert_eq!(data.len(), 1);

    // get ticket by username
    let response = client
        .get(format!(
            "{}/tickets/by-user/{}",
            test_app.address, test_user_id
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let data = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(id.to_string(), data[0]["id"]);
    assert_eq!("Trivium", data[0]["concert_name"]);

    // update ticket
    let response = client
        .patch(format!("{}/tickets/{}", test_app.address, id))
        .body(
            json!({
                "owner_id": test_user_id,
                "concert_name": "Not Trivium",
                "concert_date": "2021-08-03T00:00:00Z",
                "barcode_data": "12345-abcde-67890",
                "price": 55.0,
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let saved = sqlx::query!(
        "SELECT id, owner_id, concert_name, concert_date, barcode_data, price FROM Tickets"
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch from db.");
    assert_eq!(test_user_id, saved.owner_id);
    assert_eq!("Not Trivium", saved.concert_name);
    assert_eq!("12345-abcde-67890", saved.barcode_data);
    assert_eq!(55.0, saved.price);

    // update ticket
    let response = client
        .delete(format!("{}/tickets/{}", test_app.address, id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let saved = sqlx::query!("SELECT id FROM Tickets")
        .fetch_optional(&test_app.db_pool)
        .await
        .expect("Failed to fetch from db.");
    assert!(saved.is_none());
}
