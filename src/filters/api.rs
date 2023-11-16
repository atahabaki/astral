use super::with_realm_dao;
use crate::{daos::RealmDao, handlers};
use warp::Filter;

pub fn apis(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("api").and(planes(dao.clone()))
}

pub fn planes(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_all(dao.clone())
        .or(get_realm(dao.clone()))
        .or(create_realm(dao.clone()))
}

fn list_all(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_realm_dao(dao))
        .and_then(handlers::astral_plane::list_realms)
}

fn get_realm(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::get())
        .and(with_realm_dao(dao))
        .and(warp::path::param())
        .and_then(handlers::astral_plane::get_realm)
}

fn create_realm(
    dao: RealmDao,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("plane")
        .and(warp::post())
        .and(with_realm_dao(dao))
        .and(warp::body::json())
        .and_then(handlers::astral_plane::create_realm)
}
