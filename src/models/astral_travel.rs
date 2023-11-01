use crate::defaults::get_defaults;

pub struct AstralTravel {
    /// Short name such as: ap for Astral Plane, g for Google,
    /// y for YouTube, and etc.
    realm_short_name: String,
    /// Full name such as: Astral Plane, Google, DuckDuckGo, and etc.
    realm_full_name: String,
    /// homepage uri
    settlement_uri: String,
    /// search uri
    landing_uri: String,
    /// usage count
    landing_count: usize
}

impl Default for AstralTravel {
    fn default() -> Self {
        let defaults = get_defaults();
        let uri = format!("http://{}.{}.{}.{}:{}", defaults.0[0], defaults.0[1], defaults.0[2], defaults.0[3], defaults.1);
        AstralTravel { 
            realm_short_name: "ap".to_owned(),
            realm_full_name: "Astral Plane".to_owned(),
            settlement_uri: uri.clone(),
            landing_uri: format!("{}/travel?destination={{%query%}}", uri),
            landing_count: 0
        }
    }
}