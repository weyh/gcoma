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
use crate::ui::ui_traits::*;

pub struct UI {
    pub path: String,
    pub config: Config,
}

impl UI {
    fn sgs_to_vec_str(config: &Config, left_padding: usize) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        let sg_name_max_len = config
            .session_groups
            .iter()
            .map(|x| x.name.len())
            .max()
            .unwrap_or(0);

        let mut i: usize = 0;
        let mut last_i: usize;

        let mut line: String;
        let mut line_len: usize;
        let mut line_sub: String;
        let mut sg_name_len: usize;

        // this loop will cause lots of heap allocations
        for sg in &config.session_groups {
            sg_name_len = sg.name.len();
            line = config.colors.success().paint(&sg.name).to_string();
            line_len = line.len();

            last_i = i;
            for session in &sg.sessions {
                line_sub = config
                    .colors
                    .primary()
                    .paint(format!("{}: {}", i, session.name))
                    .to_string();

                line = format!(
                    "{0:>1$} {2:>3$}",
                    line,
                    left_padding + sg_name_len + (line_len - sg_name_len),
                    line_sub,
                    sg_name_max_len - sg_name_len + line_sub.len()
                )
                .to_string();
                output.push(line);

                // because none visible chars still count into the line's length
                line = config.colors.success().paint("").to_string();
                i += 1;
            }

            // incase there are no sessions added to session group
            if last_i == i {
                output.push(
                    format!(
                        "{0:>1$}",
                        line,
                        left_padding + sg_name_len + (line_len - sg_name_len)
                    )
                    .to_string(),
                );
            }
        }

        return output;
    }

    fn sessions(&self) -> Vec<&Session> {
        return self
            .config
            .session_groups
            .iter()
            .flat_map(|x| &x.sessions)
            .collect();
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

    fn load(&self) -> Config {
        let json = fs::read_to_string(&self.path).unwrap_or(String::new());
        return serde_json::from_str(&json).unwrap_or(Config::new());
    }
}

impl CUI for UI {
    fn new(path: &str) -> UI {
        let json = fs::read_to_string(path).unwrap_or(String::new());

        UI {
            path: path.to_string(),
            config: serde_json::from_str(&json).unwrap_or(Config::new()),
        }
    }

    fn save(&self) -> bool {
        let json = serde_json::to_string_pretty(&self.config).unwrap();

        return fs::write(&self.path, json).is_ok();
    }
}

impl QUI for UI {
    fn list_all_sessions(&self) {
        for line in &UI::sgs_to_vec_str(&self.config, 0) {
            println!("{}", line);
        }
    }

    fn connect_to_session_by_index(&self, index: usize) {
        let sessions = self.sessions();

        if index < sessions.len() {
            sessions[index].connect();
        } else {
            println!("{}", self.config.colors.error().paint("Invalid index!"));
        }
    }

    fn remove_session_group_by_name(&mut self, name: &str) {
        self.config.session_groups.retain(|x| x.name != name.trim());
        self.save();
    }
}

impl FUI for UI {
    fn main_menu(&mut self) {
        let menu_text = self
            .config
            .colors
            .primary()
            .paint("q: Quit\na: Add\nr: Remove\nR: Reload");
        let mut user_input = String::new();

        loop {
            clear_screen!();
            set_cursor_position!(0, 0);

            let all_sessions = self.sessions();
            for line in &UI::sgs_to_vec_str(&self.config, 12) {
                println!("{}", line);
            }

            set_cursor_position!(0, 0);
            println!("{}", menu_text);

            set_cursor_position!(0, cmp::max(all_sessions.len(), 4));
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
                "R" => {
                    self.config = self.load();
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

    fn add_menu(&mut self) {
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

    fn remove_menu(&mut self) {
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

        self.remove_session_group_by_name(user_input.trim());
    }
}
