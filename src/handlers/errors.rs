use serde::Serialize;
use serde_json::json;
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
};

use crate::domain::errors::{Error, Result};

pub fn result_to_warp_reply<T>(result: Result<T>) -> Result<impl Reply, Rejection>
where
    T: Serialize,
{
    match result {
        Ok(data) => Ok(reply::with_status(reply::json(&data), StatusCode::ACCEPTED)),

        Err(e) => {
            let message = e.to_string();
            Ok(reply::with_status(
                reply::json(&json!({
                    "status": "fail",
                    "message": message,
                })),
                to_http_status_code(e),
            ))
        }
    }
}

fn to_http_status_code(e: Error) -> StatusCode {
    match e {
        Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::UsernameAlreadyExists => StatusCode::CONFLICT,
        Error::UserNotFound => StatusCode::NO_CONTENT,
        Error::UserCreationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::UserFetchFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::UserUpdateFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::UserDeletionFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::TicketFetchFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::TicketNotFound => StatusCode::NO_CONTENT,
        Error::TicketCreationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::TicketUpdateFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::TicketDeletionFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::InvalidPassword(_) => StatusCode::BAD_REQUEST,
        Error::InvalidUsername(_) => StatusCode::BAD_REQUEST,
        Error::InvalidRole(_) => StatusCode::BAD_REQUEST,
        Error::LoginFailed(_) => StatusCode::UNAUTHORIZED,
    }
}
