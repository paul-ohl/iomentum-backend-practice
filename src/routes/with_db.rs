use std::{convert::Infallible, sync::Arc};

use sqlx::PgPool;
use warp::Filter;

/// This function is used to pass the database connection to the handler functions
///
/// example usage:
/// ```
/// warp::path("tickets")
///    .and(warp::get())
///    .and(with_db(db))
///    .and_then(tickets::get_all_tickets)
/// ```
///
/// The `get_all_tickets` function will have access to the database connection:
/// ```
/// async fn get_all_tickets(db: sqlx::PgPool) -> Result<impl warp::Reply, Infallible> {
///    ...
/// }
/// ```
pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
