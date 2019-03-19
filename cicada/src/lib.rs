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

//const SEP: char = '\u{001f}';
const SEP: char = ' ';

#[derive(Debug, Clone)]
struct ParseError;

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

enum Command {
    GET {key: String},
    INSERT {key: String, value: String}
}

enum Response {
    OK (Option<String>),
    ERR (Option<String>)
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

impl ToString for Response {
    fn to_string(&self) -> String {
        match self {
            Response::OK(msg) => {
                "ok".to_string()
            },
            Response::ERR(msg) => {
                "err".to_string()
            }
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
            Ok(stream) => {
                let map = Arc::clone(&map);

                pool.assign(|| {
                    handle_request(stream, map);
                });
            },

            Err(e) => {
                error!("Unable to accept connection: {}", e);
                continue;
            }
        };
    }
}

fn handle_request(mut stream: TcpStream, map: Arc<CHashMap<String, String>>) -> Result<()> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer)?;

    debug!("{}", string);

    let command = string.parse()?;
    let response = execute(command, map);

    write_response(&mut stream, response)?;

    Ok(())
}

fn execute(command: Command, map: Arc<CHashMap<String, String>>) -> Response {
    match command {
        Command::GET {key} => {
            if let Some(rg) =  map.get(&key) {
                Response::OK(Some(rg.to_string()))
            } else {
                Response::ERR(Some("Key not found.".to_string()))
            }
        },

        Command::INSERT {key, value} => {
            if let Some(old) = map.insert(key, value) {
                Response::OK(Some(old))
            } else {
                Response::OK(None)
            }
        }
    }
}

fn write_response(stream: &mut TcpStream, response: impl ToString) -> Result<()> {
    stream.write(response.to_string().as_bytes())?;
    stream.flush()?;
    Ok(())
}