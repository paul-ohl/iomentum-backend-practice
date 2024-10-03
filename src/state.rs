use crate::{
    handlers::jwt_handler::JwtHandler,
    models::{pg_tickets::PgTicketsModel, tickets::TicketsModel},
};

pub struct AppState<TM: TicketsModel = PgTicketsModel> {
    pub jwt_handler: JwtHandler,
    pub ticket_model: TM,
}

impl<TM> AppState<TM>
where
    TM: TicketsModel,
{
    pub fn new(jwt_secret: String, ticket_model: TM) -> Self {
        let jwt_handler = JwtHandler::new(jwt_secret).expect("cannot create jwt handler");

        AppState {
            jwt_handler,
            ticket_model,
        }
    }
}

// pub async fn create_state<TM>(db_url: String, jwt_secret: String) -> AppState<TM>
// where
//     TM: TicketsModel,
// {
//     let db_pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&db_url)
//         .await
//         .expect("cannot log to db");
//
//     let jwt_handler = JwtHandler::new(jwt_secret).expect("cannot create jwt handler");
//
//     let ticket_model = PgTicketsModel::new(db_pool.clone());
//
//     AppState {
//         jwt_handler,
//         ticket_model,
//     }
// }
