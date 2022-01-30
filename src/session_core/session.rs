use serde::{Deserialize, Serialize};
use std::process::Command;

use super::connection_type::ConnectionType;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub data: String,
    pub connection_type: ConnectionType,
}

impl Session {
    pub fn new(name: String, data: String, connection_type: ConnectionType) -> Session {
        Session {
            name,
            data,
            connection_type,
        }
    }

    pub fn get_user_name(&self) -> String {
        let end = self.data.find("@").unwrap_or(0);

        self.data[..end].to_string()
    }

    pub fn get_ip(&self) -> String {
        let mut start = self.data.find("@").unwrap_or(0);
        let end = self.data.find(":").unwrap_or(self.data.len());

        if start != 0 {
            start += 1;
        }

        self.data[start..end].to_string()
    }

    pub fn get_port(&self) -> String {
        let start = self.data.find(":").unwrap_or(0);

        if start == 0 && self.connection_type == ConnectionType::SSH {
            return "22".to_string();
        } else if start == 0 && self.connection_type == ConnectionType::Telnet {
            return "23".to_string();
        }

        let end = self.data.find("/").unwrap_or(self.data.len());

        self.data[start + 1..end].to_string()
    }

    pub fn connect(&self) {
        let prog = self.connection_type.to_string();
        let mut args: Vec<String> = vec![self.get_ip()];

        match self.connection_type {
            ConnectionType::SSH => {
                args.push("-p".to_string());
                args.push(self.get_port());

                let usr_name = self.get_user_name();
                if usr_name != "" {
                    args.push("-l".to_string());
                    args.push(usr_name);
                }
            }
            ConnectionType::Telnet => {
                args.push(self.get_port());
            }
        }

        let mut child = Command::new(prog).args(&args).spawn().unwrap();
        let _ = child.wait().unwrap();
    }
}
