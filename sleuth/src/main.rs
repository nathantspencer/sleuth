use std::io::prelude::*;
use std::fs::File;

fn main() {
    let file: File = File::open("test.jpg").expect("Could not open file.");

    let mut is_segment: bool = false;
    for byte in file.bytes() {

        let unwrapped_byte = byte.unwrap();
        if is_segment && unwrapped_byte == 225 {
            println!("APP1 segment found");
        } else {
            is_segment = if unwrapped_byte == 255 { true } else { false };
        }
        
    }

}
