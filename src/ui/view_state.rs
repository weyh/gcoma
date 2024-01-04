use super::config::Config;
use ratatui::widgets::TableState;
use std::collections::HashMap;

pub struct PopupState {
    /// if true edeting is enabled
    open: bool,
    key: String,
    input: String,
    cursor_position: usize,
    temp_session_group: HashMap<String, String>,
}

pub struct ViewState {
    pub table_state: TableState,
    pub config: Config,
    pub popup_state: PopupState,
}

impl PopupState {
    pub fn new() -> PopupState {
        PopupState {
            open: false,
            key: String::new(),
            input: String::new(),
            cursor_position: 0,
            temp_session_group: HashMap::new(),
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

    pub fn reset_state(&mut self) {
        self.hide();
        self.key = String::new();
        self.input = String::new();
        self.cursor_position = 0;
        self.temp_session_group = HashMap::new();
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn submit_data(&mut self) {
        self.temp_session_group
            .insert(self.key.clone(), self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    pub fn get_temp_session_group(&self) -> &HashMap<String, String> {
        return &self.temp_session_group;
    }
}

impl ViewState {
    pub fn new(config: Config) -> ViewState {
        ViewState {
            table_state: TableState::default(),
            config,
            popup_state: PopupState::new(),
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
