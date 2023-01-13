// use thiserror crate to create error messages
extern crate thiserror;

use std::io::Read;
use std::{fs, io};

use thiserror::Error;

#[derive(Error, Debug)]
enum ReadUserNameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username(path: &str) -> Result<String, ReadUserNameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_into_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUserNameError::EmptyUsername(String::from(path)));
    }
    Ok(username);
}

fn main() {
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }
}
