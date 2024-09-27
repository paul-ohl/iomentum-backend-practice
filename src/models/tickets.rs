use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::domain::errors::{Error, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub concert_name: String,
    pub concert_date: NaiveDate,
    pub barcode_data: String,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct TicketInput {
    pub owner_id: Uuid,
    pub concert_name: String,
    pub concert_date: NaiveDate,
    pub barcode_data: String,
    pub price: f64,
}

pub async fn get_all(db_pool: Arc<PgPool>) -> Result<Vec<Ticket>> {
    let tickets: Vec<Ticket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets")
            .fetch_all(&*db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
    Ok(tickets)
}

pub async fn get_by_id(db_pool: Arc<PgPool>, id: Uuid) -> Result<Ticket> {
    let ticket: Option<Ticket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE id = $1")
            .bind(id)
            .fetch_optional(&*db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
    match ticket {
        Some(ticket) => Ok(ticket),
        None => Err(Error::TicketNotFound),
    }
}

pub async fn get_by_user(db_pool: Arc<PgPool>, username: Uuid) -> Result<Vec<Ticket>> {
    let ticket: Vec<Ticket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE owner_id = $1")
            .bind(username)
            .fetch_all(&*db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
    if ticket.is_empty() {
        return Err(Error::TicketNotFound);
    }
    Ok(ticket)
}

pub async fn create(db_pool: Arc<PgPool>, new_ticket: TicketInput) -> Result<Uuid> {
    let created_id = sqlx::query!("INSERT INTO tickets (owner_id, concert_name, concert_date, barcode_data, price) VALUES ($1, $2, $3, $4, $5) returning id",
            new_ticket.owner_id,
            new_ticket.concert_name,
            new_ticket.concert_date,
            new_ticket.barcode_data,
            new_ticket.price
        )
            .fetch_one(&*db_pool)
            .await
            .map_err(Error::TicketCreationFailed)?
            .id;
    Ok(created_id)
}

pub async fn update(db_pool: Arc<PgPool>, id: Uuid, new_ticket: TicketInput) -> Result<Uuid> {
    let updated_id = sqlx::query!("UPDATE tickets SET owner_id = $1, concert_name = $2, concert_date = $3, barcode_data = $4, price = $5, updated_at = $6 WHERE id = $7 returning id",
            new_ticket.owner_id,
            new_ticket.concert_name,
            new_ticket.concert_date,
            new_ticket.barcode_data,
            new_ticket.price,
            Utc::now(),
            id
        )
            .fetch_one(&*db_pool)
            .await
            .map_err(Error::TicketUpdateFailed)?
            .id;
    Ok(updated_id)
}

pub async fn delete_one(db_pool: Arc<PgPool>, id: Uuid) -> Result<Uuid> {
    let deleted_id = sqlx::query!("DELETE FROM tickets WHERE id = $1 returning id", id)
        .fetch_one(&*db_pool)
        .await
        .map_err(Error::TicketDeletionFailed)?
        .id;
    Ok(deleted_id)
}
