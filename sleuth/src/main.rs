use std::env;
use std::fs::File;
use std::process;
use std::io::prelude::*;

fn main() {

    // Collect arguments, exit with help info if no path given
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("\tWelcome to sleuth!");
        println!("\tUsage: sleuth filename.jpg");
        process::exit(0x0F00);
    }

    // Open the provided path
    let ref arg1 = &args[1];
    let file: File = File::open(arg1).expect("Could not open file.");

    // Unload bytes from file into a Vec
    let mut bytes = Vec::new();
    for byte in file.bytes() {
        bytes.push(byte.unwrap());
    }

    let app1 = get_app1_segment(bytes);

    let app1_size = app1.len();
    println!("{}", app1_size);

    //let byte = wrapped_byte.unwrap();
    //if last_byte == 255 && byte == 225 {
    //    println!("APP1 segment found");
    //}
    //last_byte = byte;

}

fn get_app1_segment(bytes: Vec<u8>) -> Vec<u8> {

    let mut app1: Vec<u8> = Vec::new();

    let mut byte_counter = 0;

    let mut last_byte = 0;
    let mut size_byte1: u32 = 0;
    let mut size_byte2: u32 = 0;

    let mut size_byte1_set: bool = false;
    let mut size_byte2_set: bool = false;
    let mut in_app1: bool = false;

    for byte in bytes {

        if byte == 0xE1 && last_byte == 0xFF {
            app1.push(0xFF);
            app1.push(0xE1);
            in_app1 = true;
            last_byte = 0xE1;
            continue;
        }



        if in_app1 && !size_byte1_set {
            size_byte1 = byte as u32;
            size_byte1_set = true;
            app1.push(byte);
            println!("first size byte {}", size_byte1);
        } else if in_app1 && !size_byte2_set {
            size_byte2 = byte as u32;
            size_byte2_set = true;
            app1.push(byte);
            byte_counter = 2;
            println!("first size byte {}", size_byte2);
        } else if in_app1 && byte_counter < size_byte1 * 255 + size_byte2 {
            app1.push(byte);
            byte_counter = byte_counter + 1;
        }

        last_byte = byte;

    }

    return app1;

}
