//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use yocto::args::Config;
use std::io::prelude::*;
use log::LogLevelFilter;
use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::str;

const SEP: char = '\u{1f}';

#[test]
fn invalid_command() {
    bootstrap(1);
    let res = send("ABCDE".to_string());
    assert_error(res);
}

#[test]
fn invalid_number_args_1() {
    bootstrap(1);
    let res = send(format!("GET{}key{}value", SEP, SEP));
    assert_error(res);
}

#[test]
fn invalid_number_args_2() {
    bootstrap(1);
    let res = send(format!("INSERT{}key", SEP));
    assert_error(res);
}

#[test]
fn unknown_key() {
    bootstrap(1);
    let res = send(format!("GET{}key", SEP));
    assert_ok(res, None);
}

#[test]
fn insert() {
    bootstrap(1);
    let res = send(format!("INSERT{}key{}value", SEP, SEP));
    assert_ok(res, None);
}

#[test]
fn insert_retain() {
    bootstrap(2);
    let _ = send(format!("INSERT{}key{}value", SEP, SEP));
    let res = send(format!("GET{}key", SEP));
    assert_ok(res, Some("value".to_string()));
}

#[test]
fn insert_retain_spaces() {
    bootstrap(2);
    let _ = send(format!("INSERT{}ke y{}valu e", SEP, SEP));
    let res = send(format!("GET{}ke y", SEP));
    assert_ok(res, Some("valu e".to_string()));
}

#[test]
fn replace_return_old() {
    bootstrap(2);
    let _ = send(format!("INSERT{}key{}value", SEP, SEP));
    let res = send(format!("INSERT{}key{}new_value", SEP, SEP));
    assert_ok(res, Some("value".to_string()));
}

#[test]
fn replace_retain() {
    bootstrap(3);
    let _ = send(format!("INSERT{}key{}value", SEP, SEP));
    let _ = send(format!("INSERT{}key{}new_value", SEP, SEP));
    let res = send(format!("GET{}key", SEP));
    assert_ok(res, Some("new_value".to_string()));
}

#[test]
fn remove() {
    bootstrap(2);
    let _ = send(format!("INSERT{}key{}value", SEP, SEP));
    let res = send(format!("REMOVE{}key", SEP));
    assert_ok(res, Some("value".to_string()));
}

#[test]
fn remove_unknown() {
    bootstrap(1);
    let res = send(format!("REMOVE{}key", SEP));
    assert_error(res);
}

#[test]
fn clear() {
    bootstrap(2);
    let _ = send(format!("INSERT{}key{}value", SEP, SEP));
    let res = send(format!("GET{}key", SEP));
    assert_ok(res, Some("value".to_string()));
}

#[test]
fn test() {
    bootstrap(1);
    let res = send("TEST".to_string());
    assert_ok(res, None);
}

fn bootstrap(exit_after: usize) {
    let config = Config {
        threads: 1,
        iface: "127.0.0.1:7002".to_string(),
        log_level: LogLevelFilter::Error,
        exit_after: Some(exit_after)
    };

    let handle = thread::spawn(|| {
        yocto::run(config);
    });

    // give it some time to start
    thread::sleep(Duration::from_millis(200));
}

fn send(request: String) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:7002").unwrap();

    stream.write(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer);

    str::from_utf8(&buffer[..])
        .unwrap()
        .trim_end_matches(char::from(0))
        .to_string()
}

fn assert_error(response: String) {
    let split: Vec<&str> = response.split(SEP).collect();
    if split[0] != "ERR" {
        panic!("No ERR code sent.");
    }
    if split.len() < 2 {
        panic!("No error message sent.");
    }
}

fn assert_ok(response: String, with_value: Option<String>) {
    let split: Vec<&str> = response.split(SEP).collect();
    if split[0] != "OK" {
        panic!("No OK code sent.");
    }
    if let Some(value) = with_value {
        assert_eq!(value, split[1]);
    } else {
        assert_eq!(split.len(), 1);
    }
}
