use std::{fs, io};

use ui::config::Config;

#[cfg(test)]
mod tests;

mod args;
mod reqs_check;
mod session_core;
mod ui;

fn load_cfg_from_file(cfg_path: &str) -> io::Result<Config> {
    let cfg_str = fs::read_to_string(cfg_path)?;
    let config = serde_json::from_str(&cfg_str)?;
    Ok(config)
}

fn main() -> io::Result<()> {
    if !reqs_check::is_in_env("ssh") {
        panic!("'ssh' is not found in PATH!");
    }
    if !reqs_check::is_in_env("telnet") {
        panic!("'telnet' is not found in PATH!");
    }

    let matches = args::get_args();
    let cfg_path = matches.get_one::<String>("user_config");

    if cfg_path.is_some() {
        let user_config = load_cfg_from_file(cfg_path.unwrap().as_str());

        let list_flag = matches.get_one::<bool>("list").unwrap_or(&false).to_owned();
        let connect_idx = matches.get_one::<String>("connect");
        let rm_sg = matches.get_one::<String>("remove");

        if list_flag {
            let mut i = 0;

            for sg in user_config?.session_groups.iter() {
                println!("{}:", sg.name);

                for s in sg.sessions.iter() {
                    println!("  {}. {}", i, s.name);
                    i += 1;
                }
            }
        } else if connect_idx.is_some() {
            let mut idx: usize = connect_idx.unwrap().parse().unwrap();

            for sg in user_config?.session_groups.iter() {
                for s in sg.sessions.iter() {
                    if idx == 0 {
                        s.connect();
                        break;
                    }
                    idx -= 1;
                }
            }
        } else if rm_sg.is_some() {
            let mut ucfg = user_config.unwrap_or(Config::new());
            let sg_name = rm_sg.unwrap().to_owned();

            ucfg.session_groups.retain(|sg| sg.name != sg_name);

            ucfg.save(cfg_path.unwrap().as_str());
        } else {
            ui::view::display(cfg_path.unwrap().as_str(), user_config)?;
        }
    } else {
        panic!("No user config file specified!");
    }

    Ok(())
}
