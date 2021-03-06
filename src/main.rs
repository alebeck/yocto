//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use yocto::{args, logo, logger};

fn main() {
    let config = args::get();

    print!("{}", logo::LOGO);
    println!(" yocto {} - (c) 2019\n", env!("CARGO_PKG_VERSION"));

    logger::init_level(config.log_level).unwrap();

    yocto::run(config);
}
