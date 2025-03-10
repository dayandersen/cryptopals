pub fn base64_char_to_bytes(c: char) -> u8 {
    match c {
        'A'..='Z' => c as u8 - b'A',
        'a'..='z' => c as u8 - b'a' + 26,
        '0'..='9' => c as u8 - b'0' + 52,
        '+' => 62,
        '/' => 63,
        _ => panic!("We need to handle ranges better: {}", c)
    }
}

pub fn base64_str_to_bytes(inp: &str) -> Vec<u8> {
    let mut bytes:Vec<u8> = Vec::new();
    //Grab 4 chars at a time, as each char is 6 bits
    // Smoosh those 6 bits together into 3 bytes
    for chunk in inp.as_bytes().chunks(4) {
        let mut thing: u32 = 0;
        let mut count = 0;
        for byte in chunk {
             thing = thing << 6 | (base64_char_to_bytes(*byte as char) as u32); 
             count += 1;
        }

        match count {
            1 => {
                // Only got 6 bits, not enough for a full byte
                // In standard base64 this shouldn't happen, but we'll handle it
                bytes.push((thing & 0b11_1111) as u8);
            },
            2 => {
                // Got 12 bits (2 base64 chars = 12 bits = 1.5 bytes)
                bytes.push(((thing >> 4) & 0xFF) as u8);
            },
            3 => {
                // Got 18 bits (3 base64 chars = 18 bits = 2.25 bytes)
                bytes.push(((thing >> 10) & 0xFF) as u8);
                bytes.push(((thing >> 2) & 0xFF) as u8);
            },
            4 => {
                // Got 24 bits (4 base64 chars = 24 bits = 3 bytes)
                bytes.push(((thing >> 16) & 0xFF) as u8);
                bytes.push(((thing >> 8) & 0xFF) as u8);
                bytes.push((thing & 0xFF) as u8);
            },
            _ => {}
        }
    }

    bytes
}


pub fn byte_to_base64_char(b: u8) -> char {
    match b {
        0..=25 => (b'A' + b) as char,
        26..=51 => (b'a' + b - 26) as char,
        52..=61 => (b'0' + b - 52) as char,
        62 => '+',
        63 => '/',
        _ => panic!("We need to handle ranges better")
    }
}


// Converts a byte array to a corresponding base64 string
// Base64 means grabbing 6 bits at a time from the byte array and converting them
// Easiest way to do this is window with size 3 to grab 24 bits at a time 
// If the last part of the window is not full we can pad with =
pub fn bytes_to_base_64_str(add_padding: bool, inp: Vec<u8>) -> String {
    let mut ret: String = String::new();
    for byte_chunk in inp.chunks(3) {
        let mut combined: u32 = 0;
        match byte_chunk.len() {
            1 => combined |= (byte_chunk[0] as u32) << 16,
            2 => combined |= ((byte_chunk[0] as u32) << 16) | ((byte_chunk[1] as u32) << 8),
            3 => combined |= ((byte_chunk[0] as u32) << 16) | ((byte_chunk[1] as u32) << 8) | ((byte_chunk[2] as u32)),
            _ => panic!("Wait, who changed the chunk size on me?")
        }


        let map: u32 = 0b0011_1111;
        let char_1:u8 = ((combined >> 18) & map) as u8;
        let char_2:u8 = ((combined >> 12) & map) as u8;
        let char_3:u8 = ((combined >> 6) & map) as u8;
        let char_4:u8 = (combined & map) as u8;
        
        if byte_chunk.len() == 3 {
            ret.push(byte_to_base64_char(char_1));
            ret.push(byte_to_base64_char(char_2));
            ret.push(byte_to_base64_char(char_3));
            ret.push(byte_to_base64_char(char_4));
        } 

        // byte_chunk.len == 2 means we have 16 bits
        // 16 / 6  == 2 2/3, so we'll use have 3 characters
        if byte_chunk.len() == 2 {
            ret.push(byte_to_base64_char(char_1));
            ret.push(byte_to_base64_char(char_2));
            ret.push(byte_to_base64_char(char_3));
            if add_padding {
                ret.push('=');
            }
            
        }

        // byte_chunk.len == 1 means we have 8 bits
        // 8 / 6  == 1 1/3, so we'll use have 2 characters
        if byte_chunk.len() == 1 {
            ret.push(byte_to_base64_char(char_1));
            ret.push(byte_to_base64_char(char_2));
            if add_padding {
                ret.push('=');
                ret.push('=');
            }
            
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use crate::utils::binary::binary_str_to_bytes;

    use super::*;

    #[test]
    fn byte_to_base64_char_test() {
        assert_eq!(byte_to_base64_char(0), 'A');
        assert_eq!(byte_to_base64_char(1), 'B');
        assert_eq!(byte_to_base64_char(25), 'Z');
        assert_eq!(byte_to_base64_char(26), 'a');
        assert_eq!(byte_to_base64_char(51), 'z');
        assert_eq!(byte_to_base64_char(52), '0');
        assert_eq!(byte_to_base64_char(61), '9');
        assert_eq!(byte_to_base64_char(62), '+');
        assert_eq!(byte_to_base64_char(63), '/');
    }

    #[test]
    fn base64_str_to_bytes_test() {
        assert_eq!(base64_str_to_bytes("TWFu"), binary_str_to_bytes("010011010110000101101110"));
        let test_str = "Its+all+OgRe+now";
        let binary = binary_str_to_bytes("00100010 11011011 00111110 01101010 01011001 01111110 00111010 00000100 01011110 11111010 01111010 00110000");
        assert_eq!(base64_str_to_bytes(test_str), binary);
    }

    #[test]
    fn base64_char_to_bytes_test() {
        assert_eq!(base64_char_to_bytes('A'), 0);
        assert_eq!(base64_char_to_bytes('Z'), 25);
        assert_eq!(base64_char_to_bytes('a'), 26);
        assert_eq!(base64_char_to_bytes('z'), 51);
        assert_eq!(base64_char_to_bytes('0'), 52);
        assert_eq!(base64_char_to_bytes('9'), 61);
        assert_eq!(base64_char_to_bytes('+'), 62);
        assert_eq!(base64_char_to_bytes('/'), 63);
    }
}



