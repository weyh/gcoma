use clap::{App, Arg};

#[cfg(test)]
mod tests;

mod reqs_check;
mod session_core;
mod ui;

use crate::ui::cli::UI;
use crate::ui::ui_traits::*;

fn main() {
    if !reqs_check::is_in_env("ssh") {
        eprintln!("'ssh' is not found in PATH!");
        return;
    }
    if !reqs_check::is_in_env("telnet") {
        eprintln!("'telnet' is not found in PATH!");
        return;
    }

    let matches = App::new(env!("CARGO_CRATE_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("user_config")
                .short('u')
                .long("user-config")
                .help("Path to user config file")
                .value_name("USER_CONFIG")
                .required(true),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .conflicts_with_all(&["connect", "remove"])
                .help("List all sessions"),
        )
        .arg(
            Arg::new("connect")
                .short('c')
                .long("connect")
                .value_name("SESSION_INDEX")
                .conflicts_with_all(&["list", "remove"])
                .help("Connect to session by index"),
        )
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .value_name("SESSION_GROUP_NAME")
                .conflicts_with_all(&["list", "connect"])
                .help("Remove session group by name"),
        )
        .get_matches();

    let user_config = matches.value_of("user_config").unwrap();
    if user_config != "" {
        let mut ui = UI::new(user_config.trim());

        if matches.is_present("list") {
            ui.list_all_sessions();
        } else if matches.is_present("connect") {
            match matches.value_of("connect").unwrap().parse::<usize>() {
                Ok(index) => ui.connect_to_session_by_index(index),
                Err(e) => eprintln!("Couldn't parse index, {}", e),
            }
        } else if matches.is_present("remove") {
            ui.remove_session_group_by_name(matches.value_of("remove").unwrap());
        } else {
            ui.main_menu();
        }
    }
}
