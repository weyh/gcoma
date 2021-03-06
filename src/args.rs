use clap::{App, Arg};

pub fn get_args() -> clap::ArgMatches {
    return App::new(env!("CARGO_CRATE_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("user_config")
                .short('u')
                .long("user-config")
                .help("Path to user config file")
                .value_name("USER_CONFIG")
                .required(true),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .conflicts_with_all(&["connect", "remove"])
                .help("List all sessions"),
        )
        .arg(
            Arg::new("connect")
                .short('c')
                .long("connect")
                .value_name("SESSION_INDEX")
                .conflicts_with_all(&["list", "remove"])
                .help("Connect to session by index"),
        )
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .value_name("SESSION_GROUP_NAME")
                .conflicts_with_all(&["list", "connect"])
                .help("Remove session group by name"),
        )
        .get_matches();
}
