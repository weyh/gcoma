use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new(env!("CARGO_CRATE_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("user_config")
                .short('u')
                .long("user-config")
                .help("Path to user config file")
                .value_name("USER_CONFIG")
                .required(true),
        )
        .get_matches()
}
