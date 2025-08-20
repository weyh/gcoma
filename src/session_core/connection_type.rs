use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ConnectionType {
    Telnet,
    SSH,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConnectionType::Telnet => "telnet",
            ConnectionType::SSH => "ssh",
        };
        write!(f, "{}", s)
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
