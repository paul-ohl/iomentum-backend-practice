use crate::{
    handlers::jwt_handler::JwtHandler,
    models::{tickets::TicketsModel, users::UsersModel},
};

pub struct AppState {
    pub jwt_handler: JwtHandler,
    pub user_model: Box<dyn UsersModel>,
    pub ticket_model: Box<dyn TicketsModel>,
}

impl AppState {
    pub fn new(
        jwt_secret: String,
        user_model: Box<dyn UsersModel>,
        ticket_model: Box<dyn TicketsModel>,
    ) -> Self {
        let jwt_handler = JwtHandler::new(jwt_secret).expect("cannot create jwt handler");

        AppState {
            jwt_handler,
            user_model,
            ticket_model,
        }
    }
}
