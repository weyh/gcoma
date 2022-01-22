use clap::Parser;

#[cfg(test)]
mod tests;

mod reqs_check;
mod session_core;
mod ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, required = true)]
    user_config: String,
    #[clap(short, long, conflicts_with_all = &["connect", "remove"])]
    list: bool,
    #[clap(
        short,
        long,
        value_name = "SESSION_INDEX",
        default_value = "-1",
        conflicts_with_all = &["list", "remove"],
        help = "Connect to session by index"
    )]
    connect: i64,
    #[clap(
        short,
        long,
        value_name = "SESSION_INDEX",
        default_value = "",
        conflicts_with_all = &["list", "connect"],
        help = "Remove session group by name"
    )]
    remove: String,
}

fn main() {
    if !reqs_check::is_in_env("ssh") {
        eprintln!("'ssh' is not found in PATH!");
        return;
    }
    if !reqs_check::is_in_env("telnet") {
        eprintln!("'telnet' is not found in PATH!");
        return;
    }

    let args = Args::parse();

    if args.user_config != "" {
        let mut ui = ui::core_ui::UI::new(&args.user_config);

        if args.list {
            ui.print_sessions(1, true);
        } else if args.connect >= 0 {
            ui.connect_to_session(args.connect as usize);
        } else if args.remove != "" {
            ui.remove_session_group_by_name(&args.remove);
        } else {
            ui.main_menu();
        }
    }
}
