use serde::{Deserialize, Serialize};

use super::session::Session;

#[derive(Serialize, Deserialize)]
pub struct SessionGroup {
    pub name: String,
    pub sessions: Vec<Session>,
}

impl SessionGroup {
    pub fn new(name: String, sessions: Vec<Session>) -> SessionGroup {
        SessionGroup { name, sessions }
    }
}
