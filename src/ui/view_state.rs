use super::{config::Config, popup_state::PopupState};
use ratatui::widgets::TableState;

pub struct ViewState<'a> {
    pub table_state: TableState,
    pub config: Config,
    pub popup_state: PopupState<'a>,
    pub connected: bool,
}

impl<'a> ViewState<'a> {
    pub fn new(config: Config) -> ViewState<'a> {
        ViewState {
            table_state: TableState::default(),
            config,
            popup_state: PopupState::new(),
            connected: false,
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
