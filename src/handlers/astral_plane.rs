use sqlx::PgPool;

use crate::models::{realm::*, xresponse::XResponse};

pub async fn list_realms(pool: PgPool) -> Result<warp::reply::Json, warp::Rejection> {
    let planes = sqlx::query_as!(Realm, "SELECT * FROM astral_plane ORDER BY short_name")
        .fetch_all(&pool)
        .await;
    match planes {
        Ok(realms) =>
            Ok(warp::reply::json(&XResponse::without_message(&realms))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Vec<Realm>>::without_data(&e))),
    }
}

pub async fn get_realm(
    pool: PgPool,
    rs_name: String,
) -> Result<warp::reply::Json, warp::Rejection> {
    let plane = sqlx::query_as!(
        Realm,
        "SELECT * FROM astral_plane WHERE short_name = $1",
        rs_name
    )
    .fetch_one(&pool)
    .await;
    match plane {
        Ok(realm) => Ok(warp::reply::json(&XResponse::without_message(&realm))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Realm>::without_data(e))),
    }
}

pub async fn create_realm(
    pool: PgPool,
    realm: RealmCreate,
) -> Result<warp::reply::Json, warp::Rejection> {
    let sql_realm = sqlx::query_as!(
   Realm,
   "INSERT INTO astral_plane (short_name, full_name, settlement_uri, landing_uri) VALUES ($1, $2, $3, $4) RETURNING *",
   realm.short_name,
   realm.full_name,
   realm.settlement_uri,
   realm.landing_uri
   )
    .fetch_one(&pool)
    .await;
    match sql_realm {
        Ok(realm) => Ok(warp::reply::json(&XResponse::without_message(&realm))),
        Err(e) => Ok(warp::reply::json(&XResponse::<Realm>::without_data(&e))),
    }
}
