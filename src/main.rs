pub mod daos;
pub mod defaults;
pub mod filters;
pub mod handlers;
pub mod models;

use daos::RealmDao;
use defaults::get_defaults;
use sqlx::postgres::PgPoolOptions;
use warp::Filter;

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("ERR: DATABASE_URL env var must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("ERR: failed to connect to database."));
    let realm_dao = RealmDao::new(pool);
    let traveler_asks = filters::traveler_asks(realm_dao.clone());
    let astral_planes = filters::api::apis(realm_dao);
    let routes = traveler_asks.or(astral_planes);
    warp::serve(routes).run(get_defaults()).await;
}
