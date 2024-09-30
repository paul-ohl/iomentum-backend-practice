use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::models::tickets::{self, TicketInput};

use super::errors::result_to_warp_reply;

type ReplyRes<T> = Result<T, Rejection>;

pub async fn get_all_tickets(db: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let tickets = tickets::get_all(db).await;
    result_to_warp_reply(tickets)
}

pub async fn get_ticket_by_id(id: Uuid, db: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let ticket = tickets::get_by_id(db, id).await;
    result_to_warp_reply(ticket)
}

pub async fn get_ticket_by_user_id(user_id: Uuid, db: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let tickets = tickets::get_by_user(db, user_id).await;
    result_to_warp_reply(tickets)
}

pub async fn create_ticket(ticket: TicketInput, db: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::create(db, ticket).await;
    result_to_warp_reply(ticket_id)
}

pub async fn update_ticket(
    id: Uuid,
    ticket_input: TicketInput,
    db: Arc<PgPool>,
) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::update(db, id, ticket_input).await;
    result_to_warp_reply(ticket_id)
}

pub async fn delete_ticket(id: Uuid, db: Arc<PgPool>) -> ReplyRes<impl Reply> {
    let ticket_id = tickets::delete_one(db, id).await;
    result_to_warp_reply(ticket_id)
}
