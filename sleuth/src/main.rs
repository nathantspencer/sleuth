use std::env;
use std::fs::File;
use std::process;
use std::io::prelude::*;

use exif::Exif;
mod exif;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("\tWelcome to sleuth!");
        println!("\tUsage: sleuth filename.jpg");
        process::exit(0x0F00);
    }

    let ref arg1 = &args[1];
    let file: File = File::open(arg1).expect("Could not open file.");

    let mut bytes = Vec::new();
    for byte in file.bytes() {
        bytes.push(byte.unwrap());
    }

    let mut test: Exif = Default::default();
    test.extract_exif_from_bytes(bytes);

    println!("Size of APP1 Segment: {}", test.get_size_in_bytes());
}
