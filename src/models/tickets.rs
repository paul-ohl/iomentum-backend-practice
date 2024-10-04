use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    errors::Result,
    types::ticket_types::{NewTicket, Ticket},
};

#[async_trait]
pub trait TicketsModel: Send + Sync {
    async fn get_tickets(&self) -> Result<Vec<Ticket>>;

    async fn get_ticket(&self, id: Uuid) -> Result<Ticket>;

    async fn get_tickets_by_user(&self, user_id: Uuid) -> Result<Vec<Ticket>>;

    async fn create_ticket(&self, ticket: NewTicket) -> Result<Uuid>;

    async fn update_ticket(&self, id: Uuid, ticket: NewTicket) -> Result<Uuid>;

    async fn delete_ticket(&self, id: Uuid) -> Result<()>;
}
