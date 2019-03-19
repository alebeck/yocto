//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

#[macro_use]
extern crate log;

use cicada::{args, logo};

fn main() {
    let config = args::get();

    println!("{}", logo::LOGO);
    println!("  Cicada {} - (c) 2019\n", config.version);

    logger::init_level(config.log_level).unwrap();

    cicada::run(config);
}
