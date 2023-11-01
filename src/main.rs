pub mod defaults;
pub mod models;

use defaults::get_defaults;
use serde::{Serialize, Deserialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    realm: String,
}

#[tokio::main]
async fn main() {
    let search = warp::path("travel")
        .and(warp::query::<SearchQuery>())
        .map(|sq: SearchQuery| format!("{}", sq.realm));
    warp::serve(search).run(get_defaults()).await;
}
