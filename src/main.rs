extern crate rustc_serialize;
use rustc_serialize::json;
use std::io;
use std::io::prelude::*;

#[derive(RustcDecodable, RustcEncodable)]
struct Data {
    name: String,
    age: u8,
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let decoded: Option<Data> = match json::decode(&line.unwrap()) {
            Result::Ok(val) => Some(val),
            Result::Err(_) => None
        };
        match decoded {
            None => print!(""),
            Some(val) => println!("{},{}", val.name, val.age)
        };
    }
}
