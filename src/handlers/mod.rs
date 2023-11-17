use std::str::FromStr;

use crate::{daos::RealmDao, models::shift::Shift};

pub mod astral_plane;

pub async fn give_directions(
    traveler: Shift,
    dao: RealmDao,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut realm_short_name = None;
    let mut words: Vec<&str> = traveler.to.split(' ').collect();
    for (i, word) in words.clone().iter().enumerate() {
        if word.starts_with('!') {
            realm_short_name = Some(word[1..word.len()].to_owned());
            words.remove(i);
        }
    }
    match realm_short_name {
        Some(short_name) => match dao.read(short_name).await {
            Ok(realm) => {
                let question = words.join(" ");
                Ok(warp::redirect(
                    warp::http::uri::Uri::from_str(
                        realm.landing_uri.replace("{%query%}", &question).as_str(),
                    )
                    .unwrap(),
                ))
            }
            Err(_) => todo!(),
        },
        None => todo!(),
    }
}
