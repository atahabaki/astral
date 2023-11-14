pub mod api;

use sqlx::PgPool;
use warp::Filter;

use crate::models::traveler_question::TravelerQuestion;

pub fn traveler_asks() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path("travel")
        .and(warp::get())
        .and(warp::query::<TravelerQuestion>())
        .map(|tq: TravelerQuestion| format!("And, the traveler asked where is \"{}\"", tq.realm))
}

pub fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
