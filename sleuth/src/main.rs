use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("test.jpg").expect("Could not open file.");

    for byte in f.bytes() {
        println!("{}", byte.unwrap());
    }
}
