use crate::{
    daos::RealmDao,
    models::{realm::*, xresponse::XResponse},
};

pub async fn list_realms(dao: RealmDao) -> Result<warp::reply::Json, warp::Rejection> {
    let realms = dao.list_all().await;
    match realms {
        Ok(realms) => Ok(warp::reply::json(&XResponse::without_message(&realms))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Vec<Realm>>::without_data(
            &e,
        ))),
    }
}

pub async fn get_realm(
    dao: RealmDao,
    realm_short_name: String,
) -> Result<warp::reply::Json, warp::Rejection> {
    let realm = dao.read(realm_short_name).await;
    match realm {
        Ok(realm) => Ok(warp::reply::json(&XResponse::without_message(&realm))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Realm>::without_data(e))),
    }
}

pub async fn create_realm(
    dao: RealmDao,
    realm: RealmCreate,
) -> Result<warp::reply::Json, warp::Rejection> {
    let creation = dao.create(realm).await;
    match creation {
        Ok(realm) => Ok(warp::reply::json(&XResponse::without_message(&realm))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Realm>::without_data(&e))),
    }
}
