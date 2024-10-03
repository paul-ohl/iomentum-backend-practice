use std::future::Future;

use uuid::Uuid;

use crate::domain::{
    errors::Result,
    types::ticket_types::{NewTicket, Ticket},
};

pub trait TicketsModel {
    fn get_tickets(&self) -> impl Future<Output = Result<Vec<Ticket>>> + Send;

    fn get_ticket(&self, id: Uuid) -> impl Future<Output = Result<Ticket>> + Send;

    fn get_tickets_by_user(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Ticket>>> + Send;

    fn create_ticket(&self, ticket: NewTicket) -> impl Future<Output = Result<Uuid>> + Send;

    fn update_ticket(
        &self,
        id: Uuid,
        ticket: NewTicket,
    ) -> impl Future<Output = Result<Uuid>> + Send;

    fn delete_ticket(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;
}
