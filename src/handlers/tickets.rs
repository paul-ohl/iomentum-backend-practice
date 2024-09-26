use std::{convert::Infallible, sync::Arc};

use serde_json::json;
use sqlx::PgPool;
use warp::reply;

use crate::models::tickets;

pub async fn get_all_tickets(db_pool: Arc<PgPool>) -> Result<impl warp::Reply, Infallible> {
    let tickets = tickets::get_all(db_pool).await;
    Ok(warp::reply::json(&tickets))
}

pub async fn get_ticket_by_id(
    id: uuid::Uuid,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let ticket = tickets::get_by_id(db_pool, id).await;
    match ticket {
        Some(ticket) => Ok(reply::with_status(
            reply::json(&ticket),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"Ticket not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}

pub async fn get_ticket_by_username(
    username: String,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let tickets = tickets::get_by_username(db_pool, username).await;
    Ok(warp::reply::json(&tickets))
}

pub async fn create_ticket(
    ticket_input: crate::models::tickets::TicketInput,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let ticket_id = tickets::create(db_pool, ticket_input).await;
    Ok(reply::json(&json!({
        "id": ticket_id
    })))
}

pub async fn update_ticket(
    id: uuid::Uuid,
    ticket_input: crate::models::tickets::TicketInput,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let ticket_id = tickets::update(db_pool, id, ticket_input).await;
    match ticket_id {
        Some(ticket) => Ok(reply::with_status(
            reply::json(&json!({
                "id": ticket
            })),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"Ticket not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}

pub async fn delete_ticket(
    id: uuid::Uuid,
    db_pool: Arc<PgPool>,
) -> Result<impl warp::Reply, Infallible> {
    let ticket_id = tickets::delete_one(db_pool, id).await;
    match ticket_id {
        Some(ticket_id) => Ok(reply::with_status(
            reply::json(&ticket_id),
            warp::http::StatusCode::OK,
        )),
        None => Ok(reply::with_status(
            reply::json(&"Ticket not found"),
            warp::http::StatusCode::NO_CONTENT,
        )),
    }
}
