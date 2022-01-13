use std::env;

#[cfg(test)]
mod tests;

mod session_core;
mod ui;

fn parse_args(args: &mut Vec<String>, data_path: &mut String) -> bool {
    if args.len() == 1 || (args.len() == 2 && (args[1] == "--help" || args[1] == "-h")) {
        println!("Usage: {} [data_file_path]", args[0]);
        return false;
    } else if !args[1].ends_with('/')
        && !args[1].ends_with('\\')
        && !args[1].ends_with('.')
        && args[1] != ""
    {
        *data_path = args[1].clone();
        return true;
    }

    println!(
        "{}",
        ansi_term::Colour::RGB(239, 71, 111).paint("Error: File not found.")
    );
    false
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut data_path = String::new();
    if parse_args(&mut args, &mut data_path) {
        let mut ui = ui::UI::new(&data_path);
        ui.main_menu();
    }
}
