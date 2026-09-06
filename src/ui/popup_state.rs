use std::{collections::HashMap, sync::OnceLock};

use crate::session_core::{
    connection_type::ConnectionType,
    session::{Session, SessionBuilder},
    session_group::{SessionGroup, SessionGroupBuilder},
};

#[derive(PartialEq)]
pub enum PopupStateAction<'a> {
    StoreStr(&'a String),
    Store,
    Next,
}

impl<'a> PopupStateAction<'a> {
    pub fn get_data(&self) -> &String {
        match self {
            PopupStateAction::StoreStr(s) => s,
            _ => panic!("not a string"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(u8)]
pub enum PopupBuilderState {
    SessionGroupName = 0,

    SessionName,
    SessionConnectionType,
    SessionData,
    SessionAddConfirm,

    SessionAddMore,

    SessionGroupConfirm,
    Done,
}

pub struct PopupState<'a> {
    /// if true edeting is enabled
    open: bool,
    session_group_builder: SessionGroupBuilder,
    session_builder: Option<SessionBuilder>,
    sg_state: PopupBuilderState,

    pub textarea: ratatui_textarea::TextArea<'a>,

    pub temp_session_group: Option<SessionGroup>,
}

type PromptKey = (PopupBuilderState, Option<ConnectionType>);

#[allow(clippy::let_and_return)]
fn sg_prompts() -> &'static HashMap<PromptKey, (&'static str, &'static str)> {
    static MAP: OnceLock<HashMap<PromptKey, (&'static str, &'static str)>> = OnceLock::new();

    MAP.get_or_init(|| {
        let m = HashMap::from([
            (
                (PopupBuilderState::SessionGroupName, None),
                (
                    "Session Group Name:",
                    "Enter an easy to understand name for the session group...",
                ),
            ),
            (
                (PopupBuilderState::SessionName, None),
                (
                    "Session Name:",
                    "Enter an easy to understand name for the session...",
                ),
            ),
            (
                (PopupBuilderState::SessionConnectionType, None),
                (
                    "Session Type (TELNET/SSH/COMMAND):",
                    "Enter 'telnet', 'ssh', or 'command'",
                ),
            ),
            (
                (PopupBuilderState::SessionData, None),
                ("Connection data:", "username@ip:port"),
            ),
            (
                (
                    PopupBuilderState::SessionData,
                    Some(ConnectionType::Command),
                ),
                (
                    "Command:",
                    "executable followed by arguments, for example: scp -P 2222 file host:/tmp",
                ),
            ),
            (
                (PopupBuilderState::SessionAddConfirm, None),
                ("Add session? (y/n)", ""),
            ),
            (
                (PopupBuilderState::SessionAddMore, None),
                ("Add another session? (y/n)", ""),
            ),
            (
                (PopupBuilderState::SessionGroupConfirm, None),
                ("Add session group? (y/n)", ""),
            ),
            (
                (PopupBuilderState::Done, None),
                ("Session group has been created", ""),
            ),
        ]);

        m
    })
}

impl<'a> PopupState<'a> {
    pub fn new() -> PopupState<'a> {
        PopupState {
            open: false,
            session_group_builder: SessionGroup::builder(),
            session_builder: None,

            sg_state: PopupBuilderState::SessionGroupName,
            textarea: ratatui_textarea::TextArea::default(),

            temp_session_group: None,
        }
    }

    pub fn get_state(&self) -> PopupBuilderState {
        self.sg_state
    }

    pub fn get_prompt(&self) -> (&'static str, &'static str) {
        let connection_type = self
            .session_builder
            .as_ref()
            .map(|builder| *builder.get_connection_type());
        let prompt = sg_prompts()
            .get(&(self.sg_state, connection_type))
            .or_else(|| sg_prompts().get(&(self.sg_state, None)))
            .unwrap();

        *prompt
    }

    /// returns true if the state was changed
    pub fn increment_state(&mut self, data: PopupStateAction) {
        match self.sg_state {
            PopupBuilderState::SessionGroupName => {
                if data != PopupStateAction::Next {
                    self.session_group_builder.name(data.get_data().clone());
                }

                self.sg_state = PopupBuilderState::SessionName;
            }
            PopupBuilderState::SessionName => {
                if data != PopupStateAction::Next {
                    let mut session_builder = Session::builder();
                    session_builder.name(data.get_data().clone());

                    self.session_builder = Some(session_builder);
                }

                self.sg_state = PopupBuilderState::SessionConnectionType;
            }
            PopupBuilderState::SessionConnectionType => {
                if data != PopupStateAction::Next {
                    let t = match data.get_data().to_lowercase().as_str() {
                        "t" => ConnectionType::Telnet,
                        "telnet" => ConnectionType::Telnet,
                        "c" => ConnectionType::Command,
                        "command" => ConnectionType::Command,
                        _ => ConnectionType::SSH,
                    };

                    self.session_builder.as_mut().unwrap().connection_type(t);
                }

                self.sg_state = PopupBuilderState::SessionData;
            }
            PopupBuilderState::SessionData => {
                if data != PopupStateAction::Next {
                    self.session_builder
                        .as_mut()
                        .unwrap()
                        .data(data.get_data().clone());
                }

                self.sg_state = PopupBuilderState::SessionAddConfirm;
            }
            PopupBuilderState::SessionAddConfirm => {
                if data != PopupStateAction::Next {
                    let sb = self.session_builder.take().unwrap();
                    self.session_group_builder.add_session(sb);
                }

                self.sg_state = PopupBuilderState::SessionAddMore;
            }
            PopupBuilderState::SessionAddMore => {
                if data == PopupStateAction::Next {
                    self.sg_state = PopupBuilderState::SessionGroupConfirm;
                } else {
                    self.sg_state = PopupBuilderState::SessionName;
                }
            }
            PopupBuilderState::SessionGroupConfirm => {
                if data != PopupStateAction::Next {
                    self.temp_session_group = Some(self.session_group_builder.build());
                }

                self.session_group_builder = SessionGroup::builder();
                self.session_builder = None;
                self.sg_state = PopupBuilderState::Done;
            }
            _ => {}
        }

        self.clear_textarea();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn hide(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self) {
        self.open = true;
    }

    pub fn clear_textarea(&mut self) {
        // is there no clear method?
        self.textarea.move_cursor(ratatui_textarea::CursorMove::End);
        while self.textarea.delete_char() {}
    }

    pub fn reset_state(&mut self) {
        self.clear_textarea();
        self.hide();

        self.session_group_builder = SessionGroup::builder();
        self.sg_state = PopupBuilderState::SessionGroupName;
    }
}

#[cfg(test)]
mod tests {
    use super::{PopupState, PopupStateAction};

    #[test]
    fn command_session_uses_the_command_data_prompt() {
        let mut popup = PopupState::new();
        let group_name = "Test Group".to_string();
        let session_name = "Test Session".to_string();
        let connection_type = "command".to_string();

        popup.increment_state(PopupStateAction::StoreStr(&group_name));
        popup.increment_state(PopupStateAction::StoreStr(&session_name));
        popup.increment_state(PopupStateAction::StoreStr(&connection_type));

        assert_eq!(
            popup.get_prompt(),
            (
                "Command:",
                "executable followed by arguments, for example: scp -P 2222 file host:/tmp"
            )
        );
    }
}
