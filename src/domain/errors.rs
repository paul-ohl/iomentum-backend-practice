use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("username already exists")]
    UsernameAlreadyExists,
    #[error("user not found")]
    UserNotFound,
    #[error("user creation failed")]
    UserCreationFailed(sqlx::Error),
    #[error("user fetch failed")]
    UserFetchFailed(sqlx::Error),
    #[error("user update failed")]
    UserUpdateFailed(sqlx::Error),
    #[error("could not delete user")]
    UserDeletionFailed(sqlx::Error),
    #[error("ticket fetch failed")]
    TicketFetchFailed(sqlx::Error),
    #[error("ticket not found")]
    TicketNotFound,
    #[error("ticket creation failed")]
    TicketCreationFailed(sqlx::Error),
    #[error("ticket update failed")]
    TicketUpdateFailed(sqlx::Error),
    #[error("could not delete ticket")]
    TicketDeletionFailed(sqlx::Error),
}
