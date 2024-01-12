use serde::{Deserialize, Serialize};

use super::session::{Session, SessionBuilder};

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionGroup {
    pub name: String,
    pub sessions: Vec<Session>,
}

impl SessionGroup {
    pub fn new(name: String, sessions: Vec<Session>) -> SessionGroup {
        SessionGroup { name, sessions }
    }

    pub fn builder() -> SessionGroupBuilder {
        SessionGroupBuilder::new()
    }
}

pub struct SessionGroupBuilder {
    name: String,
    sessions: Vec<SessionBuilder>,
}

impl SessionGroupBuilder {
    fn new() -> SessionGroupBuilder {
        SessionGroupBuilder {
            name: "".to_string(),
            sessions: Vec::new(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut SessionGroupBuilder {
        self.name = name;
        self
    }

    pub fn add_session(&mut self, session: SessionBuilder) -> &mut SessionGroupBuilder {
        self.sessions.push(session);
        self
    }

    pub fn build(&self) -> SessionGroup {
        let mut sessions = Vec::new();

        for session in self.sessions.iter() {
            sessions.push(session.build());
        }

        SessionGroup::new(self.name.clone(), sessions)
    }
}
