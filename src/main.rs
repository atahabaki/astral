pub(crate) mod defaults;

use defaults::get_defaults;
use serde::{Serialize, Deserialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    query: String,
}

#[tokio::main]
async fn main() {
    let search = warp::path("search")
        .and(warp::query::<SearchQuery>())
        .map(|sq: SearchQuery| format!("{}", sq.query));
    warp::serve(search).run(get_defaults()).await;
}
