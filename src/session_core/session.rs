use serde::{Deserialize, Serialize};
use std::{io, process::Command};

use super::connection_type::ConnectionType;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }

    pub fn get_user_name(&self) -> String {
        if self.connection_type == ConnectionType::Command {
            return String::new();
        }

        let end = self.data.find('@').unwrap_or(0);

        self.data[..end].to_string()
    }

    pub fn get_ip(&self) -> String {
        if self.connection_type == ConnectionType::Command {
            return self.data.clone();
        }

        let mut start = self.data.find('@').unwrap_or(0);
        let end = self.data.find(':').unwrap_or(self.data.len());

        if start != 0 {
            start += 1;
        }

        self.data[start..end].to_string()
    }

    pub fn get_port(&self) -> String {
        if self.connection_type == ConnectionType::Command {
            return String::new();
        }

        let start = self.data.find(':').unwrap_or(0);

        if start == 0 && self.connection_type == ConnectionType::SSH {
            return "22".to_string();
        } else if start == 0 && self.connection_type == ConnectionType::Telnet {
            return "23".to_string();
        }

        let end = self.data.find('/').unwrap_or(self.data.len());

        self.data[start + 1..end].to_string()
    }

    /// Builds the exact process invocation represented by this session.
    pub fn build_command(&self) -> io::Result<Command> {
        let command = match self.connection_type {
            ConnectionType::SSH => {
                let mut command = Command::new("ssh");
                command.arg(self.get_ip()).arg("-p").arg(self.get_port());

                let user_name = self.get_user_name();
                if !user_name.is_empty() {
                    command.arg("-l").arg(user_name);
                }

                command
            }
            ConnectionType::Telnet => {
                let mut command = Command::new("telnet");
                command.arg(self.get_ip()).arg(self.get_port());
                command
            }
            ConnectionType::Command => command_from_data(&self.data)?,
        };

        Ok(command)
    }

    /// Returns a copyable representation of the process invocation.
    pub fn command_display(&self) -> io::Result<String> {
        let command = self.build_command()?;
        let words = std::iter::once(command.get_program().to_string_lossy().into_owned()).chain(
            command
                .get_args()
                .map(|argument| argument.to_string_lossy().into_owned()),
        );

        Ok(shell_words::join(words))
    }

    pub fn connect(&self) -> std::io::Result<std::process::ExitStatus> {
        let mut command = self.build_command()?;
        let mut child = command.spawn()?;
        child.wait()
    }
}

fn command_from_data(data: &str) -> io::Result<Command> {
    let arguments = shell_words::split(data)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    let (program, arguments) = arguments.split_first().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "command session data cannot be empty",
        )
    })?;

    let mut command = Command::new(program);
    command.args(arguments);
    Ok(command)
}

pub struct SessionBuilder {
    name: String,
    data: String,
    connection_type: ConnectionType,
}

impl SessionBuilder {
    fn new() -> SessionBuilder {
        SessionBuilder {
            name: "".to_string(),
            data: "".to_string(),
            connection_type: ConnectionType::SSH,
        }
    }

    pub fn name(&mut self, name: String) -> &mut SessionBuilder {
        self.name = name;
        self
    }

    pub fn data(&mut self, data: String) -> &mut SessionBuilder {
        self.data = data;
        self
    }

    pub fn connection_type(&mut self, connection_type: ConnectionType) -> &mut SessionBuilder {
        self.connection_type = connection_type;
        self
    }

    pub fn get_connection_type(&self) -> &ConnectionType {
        &self.connection_type
    }

    pub fn build(&self) -> Session {
        Session::new(self.name.clone(), self.data.clone(), self.connection_type)
    }
}
