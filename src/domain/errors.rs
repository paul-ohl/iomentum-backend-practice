use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal server error: {0}")]
    InternalError(String),
    #[error("username already exists")]
    UsernameAlreadyExists,
    #[error("user not found")]
    UserNotFound,
    #[error("user creation failed: {0}")]
    UserCreationFailed(sqlx::Error),
    #[error("user fetch failed: {0}")]
    UserFetchFailed(sqlx::Error),
    #[error("user update failed: {0}")]
    UserUpdateFailed(sqlx::Error),
    #[error("could not delete user: {0}")]
    UserDeletionFailed(sqlx::Error),
    #[error("ticket fetch failed: {0}")]
    TicketFetchFailed(sqlx::Error),
    #[error("ticket not found")]
    TicketNotFound,
    #[error("ticket creation failed: {0}")]
    TicketCreationFailed(sqlx::Error),
    #[error("ticket update failed: {0}")]
    TicketUpdateFailed(sqlx::Error),
    #[error("could not delete ticket: {0}")]
    TicketDeletionFailed(sqlx::Error),
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid username: {0}")]
    InvalidUsername(String),
    #[error("invalid role: {0} does not exist")]
    InvalidRole(String),
    #[error("login failed: {0}")]
    LoginFailed(String),
}
