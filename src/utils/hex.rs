
// A Hex string is something like A01BCF23
// We need to map each of these to their corresponding hex val
// So in this case the outcome would be [10,0,1,11,12,15,2,3]
pub fn hex_str_to_bytes(inp: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    let s: Vec<char> = inp.chars().collect();

    for chunk in s.chunks(2) {
        if chunk.len() == 2{ 
            ret.push(hex_char_to_byte(chunk[0]) << 4 | hex_char_to_byte(chunk[1]))
        }
        else {
            ret.push(hex_char_to_byte(chunk[0]))
        }
        
    }
    ret
}

pub fn hex_char_array_to_bytes(inp: &[char]) -> Vec<u8> {
    let mut ret = Vec::new();

    for chunk in inp.chunks(2) {
        if chunk.len() == 2{ 
            ret.push(hex_char_to_byte(chunk[0]) << 4 | hex_char_to_byte(chunk[1]))
        }
        else {
            ret.push(hex_char_to_byte(chunk[0]))
        }
        
    }
    ret
}


// Converts a given byte to its hex character value
// TODO: Fix handling for bytes that are outside of hex range.
pub fn byte_to_hex_char(b: u8) -> char {
    match b {
        0..=9 => (b + b'0') as char,
        10..=15 => (b + b'a' - 10) as char,
        _ => panic!("Bad range supplied")
    }
    
}

// Converts the given byte stream to a stream of hex values
// We assume this byte stream is packed, so it will involve breaking items up into nibbles
// Then converting those nibbles to their corresponding char value.
pub fn bytes_to_hex_str(inp: &[u8]) -> String {
    let mut ret: String = String::new();

    for b in inp.iter() {
        let nib1 = (b & 0b1111_0000) >> 4;
        let nib2 = b & 0b0000_1111;
        ret.push(byte_to_hex_char(nib1));
        ret.push(byte_to_hex_char(nib2));

    }

    ret
}

// Converts a given hex character to its bytes value
// TODO: Fix the handling on characters that are out of range
pub fn hex_char_to_byte(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - 48,
        'A'..='F' => c as u8 - 55 ,
        'a'..='f' => c as u8 - 87,
        _ => panic!("hex_char_to_byte given value outside of accepted char range, i.e. 0-9, a-f, or A-F, char was {c}", )
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::base64::bytes_to_base_64_str;

    use super::*;


    #[test]
    fn hex_str_base_64_str_test() {
        assert_eq!(bytes_to_base_64_str(false,  &hex_str_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn hex_str_to_bytes_test() {
        assert_eq!(hex_str_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), 
        &[0b01001001, 0b00100111, 0b01101101, 0b00100000, 0b01101011, 0b01101001, 0b01101100, 0b01101100, 0b01101001, 0b01101110, 0b01100111, 0b00100000, 0b01111001, 0b01101111, 0b01110101, 0b01110010, 0b00100000, 0b01100010, 0b01110010, 0b01100001, 0b01101001, 0b01101110, 0b00100000, 0b01101100, 0b01101001, 0b01101011, 0b01100101, 0b00100000, 0b01100001, 0b00100000, 0b01110000, 0b01101111, 0b01101001, 0b01110011, 0b01101111, 0b01101110, 0b01101111, 0b01110101, 0b01110011, 0b00100000, 0b01101101, 0b01110101, 0b01110011, 0b01101000, 0b01110010, 0b01101111, 0b01101111, 0b01101101]
    );
    }

    #[test] 
    fn bytes_to_base_64_str_test() {
        assert_eq!(bytes_to_base_64_str(false, &[0b01001001, 0b00100111, 0b01101101, 0b00100000, 0b01101011, 0b01101001, 0b01101100, 0b01101100, 0b01101001, 0b01101110, 0b01100111, 0b00100000, 0b01111001, 0b01101111, 0b01110101, 0b01110010, 0b00100000, 0b01100010, 0b01110010, 0b01100001, 0b01101001, 0b01101110, 0b00100000, 0b01101100, 0b01101001, 0b01101011, 0b01100101, 0b00100000, 0b01100001, 0b00100000, 0b01110000, 0b01101111, 0b01101001, 0b01110011, 0b01101111, 0b01101110, 0b01101111, 0b01110101, 0b01110011, 0b00100000, 0b01101101, 0b01110101, 0b01110011, 0b01101000, 0b01110010, 0b01101111, 0b01101111, 0b01101101]),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(bytes_to_base_64_str(false, &[0b01001101, 0b01100001, 0b01101110]),
        "TWFu");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_1111]), "/w");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_1111,0b1111_1111,0b1111_1111]), "////");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_1111,0b1111_1111]), "//8");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_0000,0b1111_1111,0b1111_1111,0b1111_1111]), "8P///w");
        assert_eq!(bytes_to_base_64_str(false, &[]), "");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111]), "Dw");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_1111,0b1111,0b1111_1111]), "/w//");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_1111,0b1111_0000]), "//A");
        assert_eq!(bytes_to_base_64_str(false, &[0b1111_0000,0b1111_1111,0b1111_1111,0b1111_1111,0b1111_0000,0b1111_1111,0b0000_1111,0b1111_1111]), "8P////D/D/8");

        let test_str: &str = "Its+all+OgRe+now";
        assert_eq!(bytes_to_base_64_str(false, &[0b00100010, 0b11011011, 0b00111110, 0b01101010, 0b01011001, 0b01111110, 0b00111010, 0b00000100, 0b01011110, 0b11111010, 0b01111010, 0b00110000]), test_str);
    }

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
        assert_eq!(byte_to_hex_char(10), 'a');
        assert_eq!(byte_to_hex_char(15), 'f');
    }

    

    #[test] 
    fn bytes_to_hex_str_test() {
        assert_eq!(bytes_to_hex_str(&[0b1111_1111]), "ff");
        assert_eq!(bytes_to_hex_str(&[0b1111_0000]), "f0");
        assert_eq!(bytes_to_hex_str(&[0b0000_1111]), "0f");
        assert_eq!(bytes_to_hex_str(&[0b0000_1111, 0b1111_1111, 0b1111_1111]), "0fffff");
        assert_eq!(bytes_to_hex_str(&[]), "");
    }
}