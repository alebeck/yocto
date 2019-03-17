//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use clap::{Arg, ArgMatches, App, SubCommand};

pub struct Config {
    pub threads: usize,
    pub version: String,
    pub iface: String
}

pub fn get() -> Config {
    let matches = App::new("cicada: minimalistic in-memory key value store")

        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help("Number of concurrent threads"))

        .arg(Arg::with_name("iface")
            .short("i")
            .long("iface")
            .takes_value(true)
            .help("IP address and port, default 127.0.0.1:7000"))

        .get_matches();

    Config {
        threads: matches.value_of("threads").unwrap_or("4").parse().unwrap(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        iface: matches.value_of("iface").unwrap_or("127.0.0.1:7000").to_string()
    }
}