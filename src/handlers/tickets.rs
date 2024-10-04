use std::sync::Arc;

use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::{
        dtos::ticket_dtos::{TicketDto, TicketInputDto},
        errors::Error,
    },
    models::tickets::TicketsModel,
    AppState,
};

use super::errors::result_to_warp_reply;

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_tickets(app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let tickets = app_state.ticket_model.get_tickets().await;
    let tickets = match tickets {
        Ok(tickets) => Ok(tickets
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<TicketDto>>()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(tickets)
}

pub async fn get_ticket_by_id(id: Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let ticket = app_state.ticket_model.get_ticket(id).await;
    let ticket: Result<TicketDto, Error> = match ticket {
        Ok(tickets) => Ok(tickets.into()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(ticket)
}

pub async fn get_ticket_by_user_id(
    user_id: Uuid,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let tickets = app_state.ticket_model.get_tickets_by_user(user_id).await;
    let tickets = match tickets {
        Ok(tickets) => Ok(tickets
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<TicketDto>>()),
        Err(e) => Err(e),
    };
    result_to_warp_reply(tickets)
}

pub async fn create_ticket(
    ticket: TicketInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    // For the future: check if the user exists
    result_to_warp_reply(app_state.ticket_model.create_ticket(ticket.into()).await)
}

pub async fn update_ticket(
    id: Uuid,
    ticket_input: TicketInputDto,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let ticket_id = app_state
        .ticket_model
        .update_ticket(id, ticket_input.into())
        .await;
    result_to_warp_reply(ticket_id)
}

pub async fn delete_ticket(id: Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let ticket_id = app_state.ticket_model.delete_ticket(id).await;
    result_to_warp_reply(ticket_id)
}
