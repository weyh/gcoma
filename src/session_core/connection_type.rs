use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ConnectionType {
    Telnet,
    SSH,
}

impl ToString for ConnectionType {
    fn to_string(&self) -> String {
        match self {
            ConnectionType::Telnet => "telnet".to_string(),
            ConnectionType::SSH => "ssh".to_string(),
        }
    }
}

impl TryFrom<i32> for ConnectionType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConnectionType::Telnet),
            1 => Ok(ConnectionType::SSH),
            _ => Err(()),
        }
    }
}
