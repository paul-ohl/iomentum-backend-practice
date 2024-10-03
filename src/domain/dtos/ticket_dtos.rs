use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::types::ticket_types::{NewTicket, Ticket};

#[derive(Debug, Serialize)]
pub struct TicketDto {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub concert_name: String,
    pub concert_date: DateTime<Utc>,
    pub barcode_data: String,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Ticket> for TicketDto {
    fn from(ticket: Ticket) -> Self {
        TicketDto {
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

#[derive(Debug, Deserialize)]
pub struct TicketInputDto {
    pub owner_id: Uuid,
    pub concert_name: String,
    pub concert_date: DateTime<Utc>,
    pub barcode_data: String,
    pub price: f64,
}

impl From<TicketInputDto> for NewTicket {
    fn from(ticket: TicketInputDto) -> Self {
        NewTicket {
            owner_id: ticket.owner_id,
            concert_name: ticket.concert_name,
            concert_date: ticket.concert_date,
            barcode_data: ticket.barcode_data,
            price: ticket.price,
        }
    }
}
