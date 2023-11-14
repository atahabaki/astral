use serde::{Deserialize, Serialize};

use crate::defaults::get_defaults;

#[derive(Serialize, Deserialize)]
pub struct Realm {
    /// Short name such as: ap for Astral Plane, g for Google,
    /// y for YouTube, and etc.
    pub short_name: String,
    /// Full name such as: Astral Plane, Google, DuckDuckGo, and etc.
    pub full_name: String,
    /// homepage uri
    pub settlement_uri: String,
    /// search uri, replace "{%query%}"
    pub landing_uri: String,
    /// usage count
    pub landing_count: i16,
}

#[derive(Serialize, Deserialize)]
pub struct RealmCreate {
    /// Short name such as: ap for Astral Plane, g for Google,
    /// y for YouTube, and etc.
    pub short_name: String,
    /// Full name such as: Astral Plane, Google, DuckDuckGo, and etc.
    pub full_name: String,
    /// homepage uri
    pub settlement_uri: String,
    /// search uri, replace "{%query%}"
    pub landing_uri: String,
}

impl Default for Realm {
    fn default() -> Self {
        let defaults = get_defaults();
        let uri = format!(
            "http://{}.{}.{}.{}:{}",
            defaults.0[0], defaults.0[1], defaults.0[2], defaults.0[3], defaults.1
        );
        Realm {
            short_name: "ap".to_owned(),
            full_name: "Astral Plane".to_owned(),
            settlement_uri: uri.clone(),
            landing_uri: format!("{}/travel?realm={{%query%}}", uri),
            landing_count: 0,
        }
    }
}
