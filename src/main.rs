#[macro_use]
extern crate lazy_static;
extern crate users;

mod token;
mod lexer;
mod repl;


use users::get_current_username;
use std::io::{stdin, stdout};

fn main() {
    let user_name = get_current_username().unwrap_or(String::from("Unknown User"));
    println!(
        "Hello {}! This is the Loris programming language!",
        user_name
    );
    println!("Feel free to type in commands.");

    let stdin = stdin();
    let in_handle = stdin.lock();

    let stdout = stdout();
    let out_handle = stdout.lock();
    if let Err(err) = repl::start(in_handle, out_handle) {
        println!("Error occured. detail: {}", err);
    }
}
