use crate::session_core::{
    connection_type::ConnectionType,
    session::{Session, SessionBuilder},
    session_group::{SessionGroup, SessionGroupBuilder},
};

use super::config::Config;
use ratatui::widgets::TableState;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

impl From<u8> for PopupBuilderState {
    fn from(val: u8) -> Self {
        match val {
            0 => PopupBuilderState::SessionGroupName,
            1 => PopupBuilderState::SessionName,
            2 => PopupBuilderState::SessionConnectionType,
            3 => PopupBuilderState::SessionData,
            4 => PopupBuilderState::SessionAddConfirm,
            5 => PopupBuilderState::SessionAddMore,
            6 => PopupBuilderState::SessionGroupConfirm,
            _ => panic!("Invalid PopupBuilderState"),
        }
    }
}

const NEW_SG_PROMPTS: [(&str, &str); (PopupBuilderState::Done as usize) + 1] = [
    (
        "Session Group Name:",
        "Enter an easy to understand name for the session group...",
    ), // SessionGroupName
    (
        "Session Name:",
        "Enter an easy to understand name for the session...",
    ), // SessionName
    (
        "Session Type (TELNET/SSH):",
        "Enter either 'telnet' or 'ssh'",
    ), // SessionConnectionType
    ("Connection data:", "username@ip:port"), // SessionData
    ("Add session? (y/n)", ""),               // SessionAddConfirm
    ("Add another session? (y/n)", ""),       // SessionAddMore
    ("Add session group? (y/n)", ""),         // SessionGroupConfirm
    ("Session group has been created", ""),   // Done
];

pub struct PopupState<'a> {
    /// if true edeting is enabled
    open: bool,
    session_group_builder: SessionGroupBuilder,
    session_builder: Option<SessionBuilder>,
    sg_prompt_index: usize,
    pub textarea: tui_textarea::TextArea<'a>,

    pub temp_session_group: Option<SessionGroup>,
}

pub struct ViewState<'a> {
    pub table_state: TableState,
    pub config: Config,
    pub popup_state: PopupState<'a>,
}

impl<'a> PopupState<'a> {
    pub fn new() -> PopupState<'a> {
        PopupState {
            open: false,
            session_group_builder: SessionGroup::builder(),
            session_builder: None,
            sg_prompt_index: 0,
            textarea: tui_textarea::TextArea::default(),

            temp_session_group: None,
        }
    }

    pub fn get_state(&self) -> PopupBuilderState {
        PopupBuilderState::from(self.sg_prompt_index as u8)
    }

    pub fn get_prompt(&self) -> Option<(&'static str, &'static str)> {
        if self.sg_prompt_index < NEW_SG_PROMPTS.len() {
            return Some(NEW_SG_PROMPTS[self.sg_prompt_index]);
        }

        None
    }

    pub fn increment_prompt(&mut self) {
        self.clear_textarea();
        self.sg_prompt_index += 1;
    }

    pub fn push_data(&mut self, data: &String) {
        match PopupBuilderState::from(self.sg_prompt_index as u8) {
            PopupBuilderState::SessionGroupName => {
                self.session_group_builder.name(data.clone());
            }
            PopupBuilderState::SessionName => {
                let mut session_builder = Session::builder();
                session_builder.name(data.clone());

                self.session_builder = Some(session_builder);
            }
            PopupBuilderState::SessionConnectionType => {
                let t = if data.to_lowercase() == "telnet" {
                    ConnectionType::Telnet
                } else {
                    ConnectionType::SSH
                };
                self.session_builder.as_mut().unwrap().connection_type(t);
            }
            PopupBuilderState::SessionData => {
                self.session_builder.as_mut().unwrap().data(data.clone());
            }
            PopupBuilderState::SessionAddConfirm => {
                let sb = self.session_builder.take().unwrap();
                self.session_group_builder.add_session(sb);
            }
            PopupBuilderState::SessionAddMore => {
                self.sg_prompt_index = PopupBuilderState::SessionName as usize;
            }
            PopupBuilderState::SessionGroupConfirm => {
                self.temp_session_group = Some(self.session_group_builder.build());
                self.session_group_builder = SessionGroup::builder();
                self.session_builder = None;
            }
            _ => {}
        }
    }

    pub fn is_open(&self) -> bool {
        return self.open;
    }

    pub fn hide(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self) {
        self.open = true;
    }

    pub fn clear_textarea(&mut self) {
        // is there no clear method?
        self.textarea.move_cursor(tui_textarea::CursorMove::End);
        while self.textarea.delete_char() {}
    }

    pub fn reset_state(&mut self) {
        self.clear_textarea();
        self.hide();

        self.session_group_builder = SessionGroup::builder();
        self.sg_prompt_index = 0;
    }
}

impl<'a> ViewState<'a> {
    pub fn new(config: Config) -> ViewState<'a> {
        ViewState {
            table_state: TableState::default(),
            config,
            popup_state: PopupState::new(),
        }
    }

    pub fn add_temp_session_group_to_cfg(&mut self) {
        if let Some(sg) = self.popup_state.temp_session_group.take() {
            self.config.session_groups.push(sg);
        }
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= (self.config.session_groups.len() - 1)
                    + (self
                        .config
                        .session_groups
                        .iter()
                        .map(|s| s.sessions.len())
                        .sum::<usize>())
                {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    (self.config.session_groups.len() - 1)
                        + (self
                            .config
                            .session_groups
                            .iter()
                            .map(|s| s.sessions.len())
                            .sum::<usize>())
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }
}
