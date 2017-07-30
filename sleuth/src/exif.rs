pub struct Exif {
    size_in_bytes: u16
}

pub fn exif_from_bytes(bytes: Vec<u8>) -> Option<Exif> {
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
        } else if in_app1 && !size_byte2_set {
            size_byte2 = byte as u32;
            size_byte2_set = true;
            app1.push(byte);
            byte_counter = 2;
        } else if in_app1 && byte_counter < size_byte1 * 255 + size_byte2 {
            app1.push(byte);
            byte_counter = byte_counter + 1;
        }

        last_byte = byte;
    }

    return Some( Exif { size_in_bytes: byte_counter as u16 } );
}

impl Exif {
    pub fn get_size_in_bytes(&self) -> u16 {
        self.size_in_bytes
    }
}
