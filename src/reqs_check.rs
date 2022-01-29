use std::env;
use std::fs;

#[cfg(target_family = "windows")]
pub fn is_in_env(prog_name: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(";") {
            // assuming that the binary has an extension
            for ext in env::var("PATHEXT").unwrap_or("exe".to_string()).split(";") {
                let p_str = format!("{}{}{}", p, prog_name, ext);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }

                let p_str_slash = format!("{}\\{}{}", p, prog_name, ext);
                if fs::metadata(p_str_slash).is_ok() {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(target_family = "unix")]
pub fn is_in_env(prog_name: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, prog_name);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}
