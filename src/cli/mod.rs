use clap::{App, Arg};

/// Starts CLI commands, subcommands and arguments
pub fn start_cli() -> clap::ArgMatches<'static> {
    let matches = App::new("Marvin - Metrics Tracker")
        .version("0.1")
        .author("Joao Henrique Machado Silva. <joaoh82@gmail.com>")
        .about("Tracks metrics running as a daemon")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets a custom output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("USER")
                .help("Sets a user to run the daemon with")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("group")
                .short("g")
                .long("group")
                .value_name("GROUP")
                .help("Sets a group to run the daemon with")
                .takes_value(true),
        )
        .get_matches();

    matches
}
