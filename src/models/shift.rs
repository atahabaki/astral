use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Shift {
    pub to: String,
}
