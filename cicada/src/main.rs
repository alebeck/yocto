//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

#[macro_use]
extern crate log;

mod args;
mod spin;
mod threadp;
mod logo;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use chashmap::{CHashMap};

fn main() {
    let config = args::get();

    println!("{}", logo::LOGO);
    println!("  Cicada {} - (c) 2019\n", config.version);

    logger::init_to_defaults().unwrap();

    let listener = match TcpListener::bind(&config.iface) {
        Ok(l) => {
            info!("Successfully bound to {}", config.iface);
            l
        },
        Err(e) => {
            error!("Failed binding to {}: {}", config.iface, e);
            std::process::exit(1);
        }
    };

    let map: CHashMap<String, String> = CHashMap::new();

    let pool = threadp::ThreadPool::new(config.threads);
    info!("Initialized thread pool with {} worker threads", config.threads);




}
