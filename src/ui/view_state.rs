use super::config::Config;
use ratatui::widgets::TableState;

pub struct ViewState {
    pub show_popup: bool,
    pub table_state: TableState,
    pub config: Config,
}

impl ViewState {
    pub fn new(config: Config) -> ViewState {
        ViewState {
            show_popup: false,
            table_state: TableState::default(),
            config,
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
