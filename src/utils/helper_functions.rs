
// A Hex string is something like A01BCF23
// We need to map each of these to their corresponding hex val
// So in this case the outcome would be [10,0,1,11,12,15,2,3]
pub fn hex_str_to_bytes(inp: &str) -> Vec<u8> {
    let mut ret = Vec::new();

    for c in inp.chars() {
        ret.push(hex_char_to_byte(c))
    }
    return ret
}

// Convert the base64 character to its corresponding val
// TODO: Better handling for characters with val greater that 63
pub fn base64_char_to_bytes(c: char) -> u8 {
    return match c {
        'A'..='Z' => c as u8 - 'A' as u8,
        'a'..='z' => c as u8 - 'a' as u8 + 26,
        '0'..='9' => c as u8 - '0' as u8 + 52,
        '+' => 62,
        '/' => 63,
        _ => panic!("We need to handle ranges better")
    };
}

// Convert the base64 character to its corresponding val
// TODO: Better handling for characters with val greater that 63
pub fn base64_str_to_bytes(inp: &str) -> Vec<u8> {
    let mut ret:Vec<u8> = Vec::new();
    for c in inp.chars() {
        ret.push(base64_char_to_bytes(c));
    }

    return ret;
    
}

pub fn byte_to_base64_char(b: u8) -> char {
    return match b {
        0..=25 => (('A' as u8) + b) as char,
        26..=51 => (('a' as u8) + b - 26) as char,
        52..=61 => (('0' as u8) + b - 52) as char,
        62 => '+',
        63 => '/',
        _ => panic!("We need to handle ranges better")
    };
}

// Converts a given byte to its hex character value
// TODO: Fix handling for bytes that are outside of hex range.
pub fn byte_to_hex_char(b: u8) -> char {
    return match b {
        0..=9 => (b + b'0') as char,
        10..=15 => (b + b'A' - 10) as char,
        _ => panic!("Bad range supplied")
    }
    
}

// Converts the given byte stream to a stream of hex values
// We assume this byte stream is packed, so it will involve breaking items up into nibbles
// Then converting those nibbles to their corresponding char value.
pub fn bytes_to_hex_str(inp: Vec<u8>) -> String {
    let mut ret: String = String::new();

    for b in inp.iter() {
        let nib1 = (b & 0b1111_0000) >> 4;
        let nib2 = b & 0b0000_1111;
        ret.push(byte_to_hex_char(nib1));
        ret.push(byte_to_hex_char(nib2));

    }

    return ret;
}

// Converts a byte array to a corresponding base64 string
// Base64 means grabbing 6 bits at a time from the byte array and converting them
// Easiest way to do this is window with size 3 to grab 24 bits at a time 
// If the last part of the window is not full we can pad with =
pub fn bytes_to_base_64_str(inp: Vec<u8>) -> String {
    let mut ret: String = String::new();
    for byte_chunk in inp.chunks(3) {
        let mut combined: u32 = 0;
        match byte_chunk.len() {
            1 => combined |= ((byte_chunk[0] as u32) << 16),
            2 => combined |= ((byte_chunk[0] as u32) << 16) | ((byte_chunk[1] as u32) << 8),
            3 => combined |= ((byte_chunk[0] as u32) << 16) | ((byte_chunk[1] as u32) << 8) | ((byte_chunk[2] as u32)),
            _ => panic!("Wait, who changed the chunk size on me?")
        }


        let map: u32 = 0b0011_1111;
        let char_1:u8 = ((combined & (map << 18)) >> 18) as u8;
        let char_2:u8 = ((combined & (map << 12)) >> 12 ) as u8;
        let char_3:u8 = ((combined & (map << 6)) >> 6) as u8;
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
        }

        // byte_chunk.len == 1 means we have 8 bits
        // 8 / 6  == 1 1/3, so we'll use have 2 characters
        if byte_chunk.len() == 1 {
            ret.push(byte_to_base64_char(char_1));
            ret.push(byte_to_base64_char(char_2));
        }
        

    }
    return ret;
}

// Converts a given hex character to its bytes value
// TODO: Fix the handling on characters that are out of range
pub fn hex_char_to_byte(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - 48,
        'A'..='F' => c as u8 - 55 ,
        'a'..='f' => c as u8 - 87,
        _ => panic!("I should handle this better :)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_char_to_byte_test() {
        assert_eq!(hex_char_to_byte('0'), 0);
        assert_eq!(hex_char_to_byte('a'), 10);
        assert_eq!(hex_char_to_byte('A'), 10);
        assert_eq!(hex_char_to_byte('f'), 15);
        assert_eq!(hex_char_to_byte('F'), 15);
    }

    #[test]
    fn byte_to_hex_char_test() {
        assert_eq!(byte_to_hex_char(0), '0');
        assert_eq!(byte_to_hex_char(1), '1');
        assert_eq!(byte_to_hex_char(10), 'A');
        assert_eq!(byte_to_hex_char(15), 'F');
        // Need to fix handling
        // assert_eq!(byte_to_hex_char(16), None);
    }

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
        // Need to fix handling
        // assert_eq!(byte_to_hex_char(16), None);
    }

    #[test]
    fn hex_str_to_bytes_test() {
        assert_eq!(hex_str_to_bytes("A01BCF23"), Vec::from([10,0,1,11,12,15,2,3]));
        assert_eq!(hex_str_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), 
        Vec::from([0b0100,0b1001,0b0010,0b0111,0b0110,0b1101,0b0010,0b0000,0b0110,0b1011,0b0110,0b1001,0b0110,0b1100,0b0110,0b1100,0b0110,0b1001,0b0110,0b1110,0b0110,0b0111,0b0010,0b0000,0b0111,0b1001,0b0110,0b1111,0b0111,0b0101,0b0111,0b0010,0b0010,0b0000,0b0110,0b0010,0b0111,0b0010,0b0110,0b0001,0b0110,0b1001,0b0110,0b1110,0b0010,0b0000,0b0110,0b1100,0b0110,0b1001,0b0110,0b1011,0b0110,0b0101,0b0010,0b0000,0b0110,0b0001,0b0010,0b0000,0b0111,0b0000,0b0110,0b1111,0b0110,0b1001,0b0111,0b0011,0b0110,0b1111,0b0110,0b1110,0b0110,0b1111,0b0111,0b0101,0b0111,0b0011,0b0010,0b0000,0b0110,0b1101,0b0111,0b0101,0b0111,0b0011,0b0110,0b1000,0b0111,0b0010,0b0110,0b1111,0b0110,0b1111,0b0110,0b1101])
    );
    }

    #[test] 
    fn bytes_to_hex_str_test() {
        assert_eq!(bytes_to_hex_str(Vec::from([0b1111_1111])), "FF");
        assert_eq!(bytes_to_hex_str(Vec::from([0b1111_0000])), "F0");
        assert_eq!(bytes_to_hex_str(Vec::from([0b0000_1111])), "0F");
        assert_eq!(bytes_to_hex_str(Vec::from([0b0000_1111, 0b1111_1111, 0b1111_1111])), "0FFFFF");
        assert_eq!(bytes_to_hex_str(Vec::from([])), "");
    }

    #[test] 
    fn bytes_to_base_64_str_test() {
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_1111])), "/w");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_1111,0b1111_1111,0b1111_1111])), "////");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_1111,0b1111_1111])), "//8");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_0000,0b1111_1111,0b1111_1111,0b1111_1111])), "8P///w");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b0000_0000])), "AA");
        assert_eq!(bytes_to_base_64_str(Vec::from([])), "");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b0000_1111])), "Dw");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_1111,0b0000_1111,0b1111_1111])), "/w//");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_1111,0b1111_0000])), "//A");
        assert_eq!(bytes_to_base_64_str(Vec::from([0b1111_0000,0b1111_1111,0b1111_1111,0b1111_1111,0b1111_0000,0b1111_1111,0b0000_1111,0b1111_1111])), "8P////D/D/8");

        let test_str = "Its+all+OgRe+now";
        assert_eq!(bytes_to_base_64_str(base64_str_to_bytes(test_str)), test_str);


        
        assert_eq!(bytes_to_base_64_str(Vec::from([0b0100,0b1001,0b0010,0b0111,0b0110,0b1101,0b0010,0b0000,0b0110,0b1011,0b0110,0b1001,0b0110,0b1100,0b0110,0b1100,0b0110,0b1001,0b0110,0b1110,0b0110,0b0111,0b0010,0b0000,0b0111,0b1001,0b0110,0b1111,0b0111,0b0101,0b0111,0b0010,0b0010,0b0000,0b0110,0b0010,0b0111,0b0010,0b0110,0b0001,0b0110,0b1001,0b0110,0b1110,0b0010,0b0000,0b0110,0b1100,0b0110,0b1001,0b0110,0b1011,0b0110,0b0101,0b0010,0b0000,0b0110,0b0001,0b0010,0b0000,0b0111,0b0000,0b0110,0b1111,0b0110,0b1001,0b0111,0b0011,0b0110,0b1111,0b0110,0b1110,0b0110,0b1111,0b0111,0b0101,0b0111,0b0011,0b0010,0b0000,0b0110,0b1101,0b0111,0b0101,0b0111,0b0011,0b0110,0b1000,0b0111,0b0010,0b0110,0b1111,0b0110,0b1111,0b0110,0b1101])),
         "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        
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