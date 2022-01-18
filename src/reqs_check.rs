use std::env;
use std::fs;

#[cfg(target_family = "windows")]
const SPLITTER: &str = ";";

#[cfg(target_family = "unix")]
const SPLITTER: &str = ":";

pub fn is_in_env(prog_name: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(SPLITTER) {
            let p_str = format!("{}/{}", p, prog_name);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}
