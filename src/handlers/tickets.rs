use std::sync::Arc;

use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    models::tickets::{self, TicketInput},
    AppState,
};

use super::errors::result_to_warp_reply;

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_tickets(app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let tickets = tickets::get_all(&app_state.db_pool).await;
    result_to_warp_reply(tickets)
}

pub async fn get_ticket_by_id(id: Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let ticket = tickets::get_by_id(&app_state.db_pool, id).await;
    result_to_warp_reply(ticket)
}

pub async fn get_ticket_by_user_id(
    user_id: Uuid,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let tickets = tickets::get_by_user(&app_state.db_pool, user_id).await;
    result_to_warp_reply(tickets)
}

pub async fn create_ticket(ticket: TicketInput, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::create(&app_state.db_pool, ticket).await;
    result_to_warp_reply(ticket_id)
}

pub async fn update_ticket(
    id: Uuid,
    ticket_input: TicketInput,
    app_state: Arc<AppState>,
) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::update(&app_state.db_pool, id, ticket_input).await;
    result_to_warp_reply(ticket_id)
}

pub async fn delete_ticket(id: Uuid, app_state: Arc<AppState>) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::delete_one(&app_state.db_pool, id).await;
    result_to_warp_reply(ticket_id)
}
