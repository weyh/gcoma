use serde::{Deserialize, Serialize};

use crate::session_core::session_group::SessionGroup;

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn save(&self, path: &str) {
        let mut data: Config = self.clone();
        data.version = env!("CARGO_PKG_VERSION").to_string();

        let cfg_str = serde_json::to_string_pretty(&data).unwrap();
        std::fs::write(path, cfg_str).unwrap();
    }
}
