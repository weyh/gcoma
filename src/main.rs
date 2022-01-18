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
    config_file_path: String,
}

fn main() {
    if !reqs_check::is_in_env("ssh") {
        println!("'ssh' is not found in PATH!");
        return;
    }
    if !reqs_check::is_in_env("telnet") {
        println!("'telnet' is not found in PATH!");
        return;
    }

    let args = Args::parse();

    if args.config_file_path != "" {
        let mut ui = ui::core_ui::UI::new(&args.config_file_path);
        ui.main_menu();
    }
}
