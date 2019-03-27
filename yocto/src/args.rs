//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use clap::{Arg, App};
use log::LogLevelFilter;

pub struct Config {
    pub threads: usize,
    pub version: String,
    pub iface: String,
    pub log_level: LogLevelFilter,
    // used for testing
    pub exit_after: Option<usize>
}

pub fn get() -> Config {
    let matches = App::new("yocto: minimalistic in-memory key value store")

        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help("Number of concurrent threads"))

        .arg(Arg::with_name("iface")
            .short("i")
            .long("iface")
            .takes_value(true)
            .help("IP address and port, default 127.0.0.1:7001"))

        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Show verbose logs"))

        .get_matches();

    Config {
        threads: matches.value_of("threads").unwrap_or("4").parse().unwrap(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        iface: matches.value_of("iface").unwrap_or("127.0.0.1:7001").to_string(),
        log_level: if matches.is_present("verbose") {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        },
        exit_after: None
    }
}