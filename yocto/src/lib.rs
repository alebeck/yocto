//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

#[macro_use]
extern crate log;

pub mod args;
pub mod logo;
mod threadp;
mod error;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{process, result, str, io};
use std::sync::Arc;
use chashmap::{CHashMap};

type Result<T> = result::Result<T, Box<std::error::Error>>;
type Response = Result<Option<String>>;
type Command = Box<Fn(Arc<CHashMap<String, String>>) -> Response>;

const SEP: char = '\u{1f}';

fn parse_command(string: String) -> Result<Command> {
    let split: Vec<String> = string.split(SEP).map(|s| s.to_string()).collect();

    match split[0].as_ref() {

        // Locates the given key inside the database and returns an Ok with the
        // corresponding value if existing or an Err if not.
        "GET" => {
            if split.len() != 2 {
                Err(Box::new(error::ParseError))
            } else {
                Ok(Box::new(move |map| {
                    if let Some(rg) = map.get(&split[1]) {
                        Ok(Some(rg.to_string()))
                    } else {
                        Err(Box::new(error::StorageError(format!("Key not found: {}", split[1]))))
                    }
                }))
            }
        },

        // Inserts a specified value at a specified key. Return the old value if existent.
        "INSERT" => {
            if split.len() != 3 {
                Err(Box::new(error::ParseError))
            } else {
                Ok(Box::new(move |map| {
                    if let Some(old) = map.insert(split[1].clone(), split[2].clone()) {
                        Ok(Some(old))
                    } else {
                        Ok(None)
                    }
                }))
            }
        },

        // Removes the value corresponding to a key. Returns Err if key is not found.
        "REMOVE" => {
            if split.len() != 2 {
                Err(Box::new(error::ParseError))
            } else {
                Ok(Box::new(move |map| {
                    if let Some(old) = map.remove(&split[1]) {
                        Ok(Some(old))
                    } else {
                        Err(Box::new(error::StorageError(format!("Key not found: {}", split[1]))))
                    }
                }))
            }
        },

        // Returns Ok("TRUE") if database contains a specified key, and Ok("FALSE") if not.
        "CONTAINS" => {
            if split.len() != 2 {
                Err(Box::new(error::ParseError))
            } else {
                Ok(Box::new(move |map| {
                    if map.contains_key(&split[1]) {
                        Ok(Some("TRUE".to_string()))
                    } else {
                        Ok(Some("FALSE".to_string()))
                    }
                }))
            }
        },

        // Removes all entries from the database.
        "CLEAR" => {
            if split.len() != 1 {
                Err(Box::new(error::ParseError))
            } else {
                Ok(Box::new(move |map| {
                    map.clear();
                    Ok(None)
                }))
            }
        },

        _ => Err(Box::new(error::ParseError))
    }
}

fn serialize(response: Response) -> String {
    match response {
        Ok(message) => {
            let mut string = "OK".to_string();
            if let Some(v) = message {
                string.push(SEP);
                string.push_str(&v);
            }
            string
        },

        Err(e) => {
            let mut string = "ERR".to_string();
            string.push(SEP);
            string.push_str(&format!("{}", e));
            string
        }
    }
}

fn handle_request(stream: &mut TcpStream, map: Arc<CHashMap<String, String>>) -> Response {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let string = str::from_utf8(&buffer[..])?
        .trim_end_matches(char::from(0))
        .to_string();

    debug!("{}", string);

    let command: Command = parse_command(string)?;
    command(map)
}

fn write_response(stream: &mut TcpStream, response: Response) -> Result<()> {
    stream.write(serialize(response).as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn run(config: args::Config) {
    let listener = match TcpListener::bind(&config.iface) {
        Ok(l) => {
            info!("Successfully bound to {}", config.iface);
            l
        },

        Err(e) => {
            error!("Failed to bind to {}: {}", config.iface, e);
            process::exit(1);
        }
    };

    let map: Arc<CHashMap<String, String>> = Arc::new(CHashMap::new());

    let pool = threadp::ThreadPool::new(config.threads);

    info!("Initialized thread pool with {} worker threads", config.threads);
    info!("Listening.");

    let iter: Box<dyn Iterator<Item=result::Result<TcpStream, io::Error>>> = if let Some(n) = config.exit_after {
        Box::new(listener.incoming().take(n))
    } else {
        Box::new(listener.incoming())
    };

    for stream in iter {
        match stream {
            Ok(mut stream) => {
                let map = Arc::clone(&map);
                pool.assign(move || {
                    let response = handle_request(&mut stream, map);

                    if let Err(e) = write_response(&mut stream, if let Err(e) = response {
                        error!("{}", e);
                        Err(e)
                    } else { response }) {
                        error!("{}", e);
                    }
                });
            },

            Err(e) => {
                error!("Unable to accept connection: {}", e);
                continue;
            }
        };
    }
}