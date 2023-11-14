use sqlx::PgPool;
use warp::Filter;
use crate::handlers;
use super::with_pool;

pub fn apis(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(planes(pool))
}

pub fn planes(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_all(pool.clone())
        .or(get_realm(pool.clone()))
        .or(create_realm(pool.clone()))
}

fn list_all(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and_then(handlers::astral_plane::list_realms)
}

fn get_realm(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(warp::path::param())
        .and_then(handlers::astral_plane::get_realm)
}

fn create_realm(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::post())
        .and(with_pool(pool.clone()))
        .and(warp::body::json())
        .and_then(handlers::astral_plane::create_realm)
}
