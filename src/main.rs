use std::io;
use std::io::prelude::*;

extern crate updated_scheme;

use updated_scheme::interpreter::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        //println!("{}", line.unwrap());
		let val = interpret(line.unwrap());
		println!("{}",val);
    }
}
