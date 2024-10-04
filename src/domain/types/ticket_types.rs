use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Ticket {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub concert_name: String,
    pub concert_date: DateTime<Utc>,
    pub barcode_data: String,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewTicket {
    pub owner_id: Uuid,
    pub concert_name: String,
    pub concert_date: DateTime<Utc>,
    pub barcode_data: String,
    pub price: f64,
}
