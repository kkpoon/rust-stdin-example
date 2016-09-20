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
        let decoded: Data = json::decode(&line.unwrap()).unwrap();
        println!("{},{}", decoded.name, decoded.age);
    }
}
