#!/usr/bin/env run_rust
use std::env;

fn main () {
    println!("Current directory: {}", env::current_dir().unwrap().to_str().unwrap());
    print!("Args:");
    env::args().for_each(|arg| print!(" {}", arg));
    println!("")
}
