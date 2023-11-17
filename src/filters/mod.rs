pub mod api;

use sqlx::PgPool;
use warp::Filter;

use crate::{daos::RealmDao, handlers, models::shift::Shift};

pub fn traveler_asks(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("shift")
        .and(warp::get())
        .and(warp::query::<Shift>())
        .and(with_realm_dao(dao.clone()))
        .and_then(handlers::give_directions)
}

pub fn with_realm_dao(
    dao: RealmDao,
) -> impl Filter<Extract = (RealmDao,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || dao.clone())
}

pub fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
