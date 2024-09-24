use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub owner_name: String,

    pub concert_name: String,
    pub concert_date: NaiveDate,
    pub barcode_data: String,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct TicketModel {
    db_pool: sqlx::PgPool,
}

impl TicketModel {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("cannot log to db");
        TicketModel { db_pool }
    }

    pub async fn get_all(&self) -> Vec<Ticket> {
        let tickets: Vec<Ticket> = sqlx::query_as("SELECT id, owner_name, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets")
            .fetch_all(&self.db_pool)
            .await
            .expect("Couldn't fetch tickets");
        tickets
    }
}
