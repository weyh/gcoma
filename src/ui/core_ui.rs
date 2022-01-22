use serde_json;
use std::cmp;
use std::fs;
use std::io::Write;
use std::{thread, time};

use crate::session_core::connection_type::ConnectionType;
use crate::session_core::session::Session;
use crate::session_core::session_group::SessionGroup;
use crate::ui::config::Config;
use crate::ui::config::UIColors;

use crate::ui::ui_macros::*;

pub struct UI {
    pub path: String,
    pub config: Config,
}

impl UI {
    pub fn new(path: &str) -> UI {
        let json = fs::read_to_string(path).unwrap_or(String::new());

        UI {
            path: path.to_string(),
            config: serde_json::from_str(&json).unwrap_or(Config::new()),
        }
    }

    pub fn save(&mut self) -> bool {
        let json = serde_json::to_string_pretty(&self.config).unwrap();

        return fs::write(&mut self.path, json).is_ok();
    }

    fn create_session(&mut self) -> Session {
        println!(
            "{}",
            self.config.colors.success().paint("--- Create Session ---")
        );

        let mut session_name = String::new();
        print_flush!("{}", self.config.colors.error().paint("Name: "));
        stdin_read_line!(&mut session_name);

        let mut op_input = String::new();
        let mut op: i32 = -1;

        while op != 0 && op != 1 {
            println!(
                "{}",
                self.config.colors.primary().paint("0. Telnet\n1. SSH")
            );

            print_flush!("{}", self.config.colors.highlight().paint("Select: "));
            stdin_read_line!(&mut op_input);

            op = op_input
                .trim()
                .replace("\"", "")
                .parse::<i32>()
                .unwrap_or(-1);
        }

        let con = ConnectionType::try_from(op).unwrap();
        let mut data = String::new();
        print_flush!(
            "{}",
            self.config.colors.primary().paint("<user>@<ip>:<port> : ")
        );
        stdin_read_line!(&mut data);

        Session::new(
            session_name.trim().to_string(),
            data.trim().to_string(),
            con,
        )
    }

    pub fn add_menu(&mut self) {
        clear_screen!();
        set_cursor_position!(0, 0);
        println!(
            "{}",
            self.config
                .colors
                .success()
                .paint("--- Create Session Group ---")
        );

        let mut user_input = String::new();

        let mut session_group_name = String::new();
        print_flush!("{}", self.config.colors.error().paint("Name: "));
        stdin_read_line!(&mut session_group_name);

        let mut sessions: Vec<Session> = Vec::new();

        loop {
            let session = self.create_session();

            print_flush!("{}", self.config.colors.primary().paint("Add? (Y/n) "));
            stdin_read_line!(&mut user_input);

            if user_input.trim() == "Y" || user_input.trim() == "y" {
                sessions.push(session);
            }

            print_flush!("{}", self.config.colors.primary().paint("Add more? (Y/n) "));
            stdin_read_line!(&mut user_input);

            if user_input.trim() == "N" || user_input.trim() == "n" {
                break;
            }
        }

        print_flush!(
            "{}",
            self.config
                .colors
                .primary()
                .paint("Add Session Group? (Y/n) ")
        );
        stdin_read_line!(&mut user_input);

        if user_input.trim() == "Y" || user_input.trim() == "y" {
            self.config.session_groups.push(SessionGroup::new(
                session_group_name.trim().to_string(),
                sessions,
            ));
        }
    }

    pub fn remove_menu(&mut self) {
        let colors: &UIColors = &self.config.colors;
        clear_screen!();
        set_cursor_position!(0, 0);
        println!("{}", colors.success().paint("--- Remove Session Group ---"));

        for session in &self.config.session_groups {
            print_flush!("{}\t", colors.primary().paint(session.name.to_string()));
        }

        let mut user_input = String::new();
        print_flush!("\n{}", colors.highlight().paint("Name: "));
        stdin_read_line!(&mut user_input);

        self.config
            .session_groups
            .retain(|x| x.name != user_input.trim());
    }

    pub fn print_sessions(&self, left_offset: usize, clear_screen: bool) -> Vec<&Session> {
        let mut all_sessions = Vec::new();
        let max_lenght = self
            .config
            .session_groups
            .iter()
            .map(|x| x.name.len())
            .max()
            .unwrap_or(0);

        if clear_screen {
            clear_screen!();
        }

        let mut h: usize = 0;
        for sg in self.config.session_groups.iter() {
            set_cursor_position!(left_offset, h + 1);
            println!("{}", self.config.colors.success().paint(&sg.name));

            for session in &sg.sessions {
                set_cursor_position!(
                    left_offset + sg.name.len() + (max_lenght - sg.name.len()) + 1,
                    h + 1
                );
                println!(
                    "{}",
                    self.config
                        .colors
                        .primary()
                        .paint(format!("{}: {}", h, session.name))
                );
                all_sessions.push(session);
                h += 1;
            }
        }

        return all_sessions;
    }

    pub fn remove_session_group_by_name(&mut self, name: &str) {
        self.config.session_groups.retain(|x| x.name != name.trim());
    }

    pub fn connect_to_session(&mut self, index: usize) {
        let mut h: usize = 0;
        for sg in self.config.session_groups.iter() {
            for session in &sg.sessions {
                if h == index {
                    session.connect();
                    return;
                }

                h += 1;
            }
        }

        println!("{}", self.config.colors.error().paint("Invalid index!"));
    }

    pub fn main_menu(&mut self) {
        let menu_text = self
            .config
            .colors
            .primary()
            .paint("q: Quit\na: Add\nr: Remove");

        let mut user_input = String::new();

        loop {
            clear_screen!();
            set_cursor_position!(0, 0);

            println!("{}", menu_text);
            let all_sessions = self.print_sessions(12, false);

            set_cursor_position!(0, cmp::max(all_sessions.len(), 3));
            print_flush!("\n\n{}", self.config.colors.highlight().paint("Select: "));
            stdin_read_line!(&mut user_input);

            match user_input.trim() {
                "q" => {
                    break;
                }
                "a" => {
                    self.add_menu();
                    self.save();
                }
                "r" => {
                    self.remove_menu();
                    self.save();
                }
                _ => {
                    if user_input.trim().parse::<usize>().is_ok() {
                        let index = user_input.trim().parse::<usize>().unwrap();
                        if index < all_sessions.len() {
                            all_sessions[index].connect();
                            thread::sleep(time::Duration::from_millis(1000));
                        }
                    }
                }
            }
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        self.save();
    }
}
