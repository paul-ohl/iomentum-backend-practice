use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::domain::{
    errors::{Error, Result},
    types::ticket_types::{NewTicket, Ticket},
};
use crate::models::tickets::TicketsModel;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PgTicket {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub concert_name: String,
    pub concert_date: DateTime<Utc>,
    pub barcode_data: String,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PgTicket> for Ticket {
    fn from(ticket: PgTicket) -> Self {
        Ticket {
            id: ticket.id,
            owner_id: ticket.owner_id,
            concert_name: ticket.concert_name,
            concert_date: ticket.concert_date,
            barcode_data: ticket.barcode_data,
            price: ticket.price,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
        }
    }
}

pub struct PgTicketsModel {
    db_pool: PgPool,
}

#[async_trait]
impl TicketsModel for PgTicketsModel {
    async fn get_tickets(&self) -> Result<Vec<Ticket>> {
        let tickets: Vec<PgTicket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets")
            .fetch_all(&self.db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
        Ok(tickets.into_iter().map(|t| t.into()).collect())
    }

    async fn get_ticket(&self, id: Uuid) -> Result<Ticket> {
        let ticket: Option<PgTicket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
        match ticket {
            Some(ticket) => Ok(ticket.into()),
            None => Err(Error::TicketNotFound),
        }
    }

    async fn get_tickets_by_user(&self, user_id: Uuid) -> Result<Vec<Ticket>> {
        let ticket: Vec<PgTicket> = sqlx::query_as("SELECT id, owner_id, concert_name, concert_date, barcode_data, price, created_at, updated_at FROM tickets WHERE owner_id = $1")
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(Error::TicketFetchFailed)?;
        if ticket.is_empty() {
            return Err(Error::TicketNotFound);
        }
        Ok(ticket.into_iter().map(|t| t.into()).collect())
    }

    async fn create_ticket(&self, new_ticket: NewTicket) -> Result<Uuid> {
        let created_id = sqlx::query!("INSERT INTO tickets (owner_id, concert_name, concert_date, barcode_data, price) VALUES ($1, $2, $3, $4, $5) returning id",
            new_ticket.owner_id,
            new_ticket.concert_name,
            new_ticket.concert_date,
            new_ticket.barcode_data,
            new_ticket.price
        )
            .fetch_one(&self.db_pool)
            .await
            .map_err(Error::TicketCreationFailed)?
            .id;
        Ok(created_id)
    }

    async fn update_ticket(&self, id: Uuid, ticket: NewTicket) -> Result<Uuid> {
        let updated_id = sqlx::query!("UPDATE tickets SET owner_id = $1, concert_name = $2, concert_date = $3, barcode_data = $4, price = $5, updated_at = $6 WHERE id = $7 returning id",
            ticket.owner_id,
            ticket.concert_name,
            ticket.concert_date,
            ticket.barcode_data,
            ticket.price,
            Utc::now(),
            id
        )
            .fetch_one(&self.db_pool)
            .await
            .map_err(Error::TicketUpdateFailed)?
            .id;
        Ok(updated_id)
    }

    async fn delete_ticket(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM tickets WHERE id = $1 returning id", id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(Error::TicketDeletionFailed)?;
        Ok(())
    }
}

impl PgTicketsModel {
    pub async fn new(db_url: String) -> std::result::Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Ok(Self { db_pool })
    }
}
