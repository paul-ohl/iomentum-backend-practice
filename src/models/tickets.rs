use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
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

#[derive(Deserialize, Debug)]
pub struct TicketInput {
    pub owner_name: String,
    pub concert_name: String,
    pub concert_date: NaiveDate,
    pub barcode_data: String,
    pub price: f64,
}

pub async fn get_all(db_pool: Arc<PgPool>) -> Vec<Ticket> {
    let tickets: Vec<Ticket> = sqlx::query_as("SELECT id, owner_name, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets")
            .fetch_all(&*db_pool)
            .await
            .expect("Couldn't fetch tickets");
    tickets
}

pub async fn get_by_id(db_pool: Arc<PgPool>, id: Uuid) -> Option<Ticket> {
    let ticket: Option<Ticket> = sqlx::query_as("SELECT id, owner_name, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE id = $1")
            .bind(id)
            .fetch_optional(&*db_pool)
            .await
            .expect("Couldn't fetch ticket");
    ticket
}

pub async fn get_by_username(db_pool: Arc<PgPool>, username: String) -> Vec<Ticket> {
    let ticket: Vec<Ticket> = sqlx::query_as("SELECT id, owner_name, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE owner_name = $1")
            .bind(username)
            .fetch_all(&*db_pool)
            .await
            .expect("Couldn't fetch ticket");
    ticket
}

pub async fn create(db_pool: Arc<PgPool>, new_ticket: TicketInput) -> Option<Uuid> {
    let res = sqlx::query!("INSERT INTO tickets (owner_name, concert_name, concert_date, barcode_data, price) VALUES ($1, $2, $3, $4, $5) returning id",
            new_ticket.owner_name,
            new_ticket.concert_name,
            new_ticket.concert_date,
            new_ticket.barcode_data,
            new_ticket.price
        )
            .fetch_one(&*db_pool)
            .await
            .expect("Couldn't create ticket")
            .id;
    Some(res)
}

pub async fn update(db_pool: Arc<PgPool>, id: Uuid, new_ticket: TicketInput) -> Option<Uuid> {
    let res = sqlx::query!("UPDATE tickets SET owner_name = $1, concert_name = $2, concert_date = $3, barcode_data = $4, price = $5, updated_at = $6 WHERE id = $7 returning id",
            new_ticket.owner_name,
            new_ticket.concert_name,
            new_ticket.concert_date,
            new_ticket.barcode_data,
            new_ticket.price,
            Utc::now(),
            id
        )
            .fetch_one(&*db_pool)
            .await
            .expect("Couldn't update ticket")
            .id;
    Some(res)
}

pub async fn delete_one(db_pool: Arc<PgPool>, id: Uuid) -> Option<Uuid> {
    let res = sqlx::query!("DELETE FROM tickets WHERE id = $1 returning id", id)
        .fetch_one(&*db_pool)
        .await
        .expect("Couldn't delete ticket")
        .id;
    Some(res)
}
