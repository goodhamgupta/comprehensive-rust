// try-operator `?` is used to return errors to the caller
// allows conversion of the following script
// match some_exp {
//  Ok(value) => value,
//  Err(err) => return Err(err)
//  }
// into a simplier
// some_exp?
// simplify error handling

use std::fs;
use std::io::{self, Read};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    };
}

fn main() {
    // fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
