use serde::{Deserialize, Serialize};

use crate::session_core::session_group::SessionGroup;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub session_groups: Vec<SessionGroup>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
            session_groups: Vec::new(),
        }
    }

    pub fn save(self, path: &str) {
        let cfg_str = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(path, cfg_str).unwrap();
    }
}
