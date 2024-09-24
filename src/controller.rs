use std::convert::Infallible;

use crate::models::tickets::TicketModel;

pub struct Controller {
    ticket_model: TicketModel,
}

impl Controller {
    pub fn new(ticket_model: TicketModel) -> Self {
        Self { ticket_model }
    }

    pub fn health() -> String {
        "OK".to_string()
    }

    pub async fn get_all_tickets(&self) -> Result<impl warp::Reply, Infallible> {
        let tickets = self.ticket_model.get_all().await;
        Ok(warp::reply::json(&tickets))
    }
}
