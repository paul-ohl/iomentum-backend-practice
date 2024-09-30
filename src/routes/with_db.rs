use std::{convert::Infallible, sync::Arc};

use sqlx::PgPool;
use warp::Filter;

/// This function is used to pass the database connection to the handler functions
pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
