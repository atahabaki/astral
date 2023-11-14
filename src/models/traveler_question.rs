use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TravelerQuestion {
    pub realm: String,
}
