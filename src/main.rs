#[cfg(test)]
mod tests;

mod args;
mod reqs_check;
mod session_core;
mod ui;

use ui::cli::UI;
use ui::ui_traits::*;

fn main() {
    if !reqs_check::is_in_env("ssh") {
        eprintln!("'ssh' is not found in PATH!");
        return;
    }
    if !reqs_check::is_in_env("telnet") {
        eprintln!("'telnet' is not found in PATH!");
        return;
    }

    let matches = args::get_args();

    let user_config = matches.value_of("user_config").unwrap();
    if user_config != "" {
        let mut ui = UI::new(user_config.trim());

        if matches.is_present("list") {
            ui.list_all_sessions();
        } else if matches.is_present("connect") {
            match matches.value_of("connect").unwrap().parse::<usize>() {
                Ok(index) => ui.connect_to_session_by_index(index),
                Err(e) => eprintln!("Couldn't parse index, {e}"),
            }
        } else if matches.is_present("remove") {
            ui.remove_session_group_by_name(matches.value_of("remove").unwrap());
        } else {
            ui.main_menu();
        }
    }
}
