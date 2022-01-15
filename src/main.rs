use clap::Parser;

#[cfg(test)]
mod tests;

mod session_core;
mod ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, required = true)]
    config_file_path: String,
}

fn main() {
    let args = Args::parse();

    if args.config_file_path != "" {
        let mut ui = ui::core_ui::UI::new(&args.config_file_path);
        ui.main_menu();
    }
}
