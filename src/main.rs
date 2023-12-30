use std::io;

#[cfg(test)]
mod tests;

mod args;
mod reqs_check;
mod session_core;
mod ui;

fn main() -> io::Result<()> {
    if !reqs_check::is_in_env("ssh") {
        panic!("'ssh' is not found in PATH!");
    }
    if !reqs_check::is_in_env("telnet") {
        panic!("'telnet' is not found in PATH!");
    }

    let matches = args::get_args();
    let user_config = matches.get_one::<String>("user_config");

    if user_config.is_some() {
        ui::view::display(user_config.unwrap())?;
    } else {
        panic!("No user config file specified!");
    }

    Ok(())
}
