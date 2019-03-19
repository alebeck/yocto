//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

#[macro_use]
extern crate log;

pub mod args;
pub mod logo;
mod threadp;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, process, io, result, error, fmt};
use std::sync::Arc;
use chashmap::{CHashMap};
use std::str::FromStr;

type Result<T> = result::Result<T, Box<error::Error>>;
type Response = Result<Option<String>>;

//const SEP: char = '\u{001f}';
const SEP: char = ' ';

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug, Clone)]
struct StorageError(String);

enum Command {
    GET {key: String},
    INSERT {key: String, value: String}
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse command")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Unable to parse command"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for StorageError {
    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for Command {
    type Err = Box<error::Error>;

    fn from_str(string: &str) -> Result<Command> {
        let split: Vec<&str> = string.split(SEP).collect();

        match split[0] {
            "GET" => {
                if split.len() != 2 {
                    Err(Box::new(ParseError))
                } else {
                    Ok(Command::GET {key: split[1].to_string()})
                }
            },

            "INSERT" => {
                if split.len() != 3 {
                    Err(Box::new(ParseError))
                } else {
                    Ok(Command::INSERT {key: split[1].to_string(), value: split[2].to_string()})
                }
            },

            _ => Err(Box::new(ParseError))
        }
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

    for stream in listener.incoming() {
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

fn handle_request(stream: &mut TcpStream, map: Arc<CHashMap<String, String>>) -> Response {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer)?;

    debug!("{}", string);

    let command = string.parse()?;
    execute(command, map)
}

fn execute(command: Command, map: Arc<CHashMap<String, String>>) -> Response {
    match command {
        Command::GET {key} => {
            if let Some(rg) =  map.get(&key) {
                Ok(Some(rg.to_string()))
            } else {
                Err(Box::new(StorageError(format!("Key not found: {}", key))))
            }
        },

        Command::INSERT {key, value} => {
            if let Some(old) = map.insert(key, value) {
                Ok(Some(old))
            } else {
                Ok(None)
            }
        }
    }
}

fn write_response(stream: &mut TcpStream, response: Response) -> Result<()> {
    stream.write(serialize(response).as_bytes())?;
    stream.flush()?;
    Ok(())
}