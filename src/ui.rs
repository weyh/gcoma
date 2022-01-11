use ansi_term::Colour::RGB;
use serde_json;
use std::cmp;
use std::fs;
use std::io::Write;
use std::{thread, time};

use crate::session_core::connection_type::ConnectionType;
use crate::session_core::session::Session;
use crate::session_core::session_group::SessionGroup;

macro_rules! clear_screen {
    () => {
        print_flush!("\x1B[2J");
    };
}

macro_rules! set_cursor_position {
    ($x:expr, $y:expr) => {
        print_flush!("\x1B[{};{}H", $y, $x);
    };
}

macro_rules! print_flush {
    ($($arg:tt)*) => {
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    };
}

macro_rules! stdin_read_line {
    ($x:expr) => {
        $x.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line($x).unwrap();
    };
}

const WHITE: ansi_term::Colour = RGB(248, 248, 242);
const GREAN: ansi_term::Colour = RGB(80, 250, 123);
const PURPLE: ansi_term::Colour = RGB(189, 147, 249);
const ORANGE: ansi_term::Colour = RGB(241, 250, 140);

pub struct UI {
    pub path: String,
    pub session_groups: Vec<SessionGroup>,
}

impl UI {
    pub fn new(path: &str) -> UI {
        let json = fs::read_to_string(path).unwrap_or(String::new());

        UI {
            path: path.to_string(),
            session_groups: serde_json::from_str(&json).unwrap_or(Vec::new()),
        }
    }

    pub fn save(&mut self) -> bool {
        let json = serde_json::to_string_pretty(&self.session_groups).unwrap();

        return fs::write(&mut self.path, json).is_ok();
    }

    fn create_session(&mut self) -> Session {
        println!("{}", GREAN.paint("--- Create Session ---"));

        let mut session_name = String::new();
        print_flush!("{}", ORANGE.paint("Name: "));
        stdin_read_line!(&mut session_name);

        let mut op_input = String::new();
        let mut op: i32 = -1;

        while op != 0 && op != 1 {
            println!("{}", WHITE.paint("0. Telnet\n1. SSH"));

            print_flush!("{}", PURPLE.paint("Select: "));
            stdin_read_line!(&mut op_input);

            op = op_input
                .trim()
                .replace("\"", "")
                .parse::<i32>()
                .unwrap_or(-1);
        }

        let con = ConnectionType::try_from(op).unwrap();
        let mut data = String::new();
        print_flush!("{}", WHITE.paint("<user>@<ip>:<port> : "));
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
        println!("{}", GREAN.paint("--- Create Session Group ---"));

        let mut user_input = String::new();

        let mut session_group_name = String::new();
        print_flush!("{}", ORANGE.paint("Name: "));
        stdin_read_line!(&mut session_group_name);

        let mut sessions: Vec<Session> = Vec::new();

        loop {
            let session = self.create_session();

            print_flush!("{}", WHITE.paint("Add? (Y/n) "));
            stdin_read_line!(&mut user_input);

            if user_input.trim() == "Y" || user_input.trim() == "y" {
                sessions.push(session);
            }

            print_flush!("{}", WHITE.paint("Add more? (Y/n) "));
            stdin_read_line!(&mut user_input);

            if user_input.trim() == "N" || user_input.trim() == "n" {
                break;
            }
        }

        print_flush!("{}", WHITE.paint("Add Session Group? (Y/n) "));
        stdin_read_line!(&mut user_input);

        if user_input.trim() == "Y" || user_input.trim() == "y" {
            self.session_groups.push(SessionGroup::new(
                session_group_name.trim().to_string(),
                sessions,
            ));
        }
    }

    pub fn remove_menu(&mut self) {
        clear_screen!();
        set_cursor_position!(0, 0);
        println!("{}", GREAN.paint("--- Remove Session Group ---"));

        for session in &self.session_groups {
            print_flush!("{}\t", WHITE.paint(session.name.to_string()));
        }

        let mut user_input = String::new();
        print_flush!("\n{}", PURPLE.paint("Name: "));
        stdin_read_line!(&mut user_input);

        self.session_groups.retain(|x| x.name != user_input.trim());
    }

    pub fn main_menu(&mut self) {
        let menu_text = WHITE.paint("q: Quit\na: Add\nr: Remove");

        let mut max_lenght = 0;

        let mut user_input = String::new();
        let mut rebuild = true;
        let mut run = true;

        while run {
            clear_screen!();
            set_cursor_position!(0, 0);

            let mut all_sessions: Vec<&Session> = Vec::new();

            if rebuild {
                max_lenght = self
                    .session_groups
                    .iter()
                    .map(|x| x.name.len())
                    .max()
                    .unwrap_or(0);

                rebuild = false;
            }

            println!("{}", menu_text);

            let mut h: usize = 0;
            for sg in self.session_groups.iter() {
                set_cursor_position!(12, h + 1);
                println!("{}", GREAN.paint(&sg.name));

                for session in &sg.sessions {
                    set_cursor_position!(
                        12 + sg.name.len() + (max_lenght - sg.name.len()) + 1,
                        h + 1
                    );
                    println!("{}", WHITE.paint(format!("{}: {}", h, session.name)));
                    all_sessions.push(session);
                    h += 1;
                }
            }

            set_cursor_position!(0, cmp::max(h, 3));
            print_flush!("\n\n{}", PURPLE.paint("Select: "));
            stdin_read_line!(&mut user_input);

            match user_input.trim() {
                "q" => {
                    run = false;
                }
                "a" => {
                    self.add_menu();
                    rebuild = true;

                    self.save();
                }
                "r" => {
                    self.remove_menu();
                    rebuild = true;

                    self.save();
                }
                _ => {
                    if user_input.trim().parse::<usize>().is_ok() {
                        let index = user_input.trim().parse::<usize>().unwrap();
                        if index < h {
                            all_sessions[index].connect();
                            thread::sleep(time::Duration::from_millis(500));
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
