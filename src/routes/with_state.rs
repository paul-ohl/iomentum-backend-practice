use std::{convert::Infallible, sync::Arc};

use warp::Filter;

use crate::AppState;

/// This function is used to pass the database connection to the handler functions
pub fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}
