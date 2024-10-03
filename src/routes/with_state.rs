use std::{convert::Infallible, sync::Arc};

use warp::Filter;

use crate::{models::tickets::TicketsModel, AppState};

/// This function is used to pass the database connection to the handler functions
pub fn with_state<TM>(
    state: Arc<AppState<TM>>,
) -> impl Filter<Extract = (Arc<AppState<TM>>,), Error = Infallible> + Clone
where
    TM: TicketsModel + Send + Sync + 'static,
{
    warp::any().map(move || state.clone())
}
