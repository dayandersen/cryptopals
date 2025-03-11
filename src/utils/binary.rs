

pub fn binary_str_to_bytes(inp: &str) -> Vec<u8> {
    let trimmed_str = inp.replace(" ", "");
    let mut ret:Vec<u8> = Vec::new();
    // We make an assumption here. 
    // The assumption is that every character passed to us only takes a single byte.
    // This is only true for a subset of characters, utf-8 strings can have characters take 1-4 bytes
    for byte_chunk in trimmed_str.as_bytes().chunks(8) {
        let mut b = 0;

        // This will be the byte for a single char
        for character_byte in byte_chunk {
            b <<= 1;
            b |= character_byte - b'0';
        }
        ret.push(b);
    }

    ret
}

pub fn get_bits(inp: u32) -> Vec<char>{
    let mut bits:Vec<char> = Vec::new();
    for i in (0..=31).rev() {
        bits.push((((inp >> i) & 1) as u8 + 48) as char  )
    }
    bits
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn binary_str_to_bytes_test() {
        assert_eq!(binary_str_to_bytes("00100010 11011011 00111110 01101010 01011001 01111110 00111010 00000100 01011110 11111010 01111010 00110000"), [0b00100010, 0b11011011, 0b00111110, 0b01101010, 0b01011001, 0b01111110, 0b00111010, 0b00000100, 0b01011110, 0b11111010, 0b01111010, 0b00110000]);
    }

    #[test]
    fn get_bits_test() {
        let mut result = vec!['0'; 24];
        result.extend(['0','1','0','1','1','0','0','1']);
        assert_eq!(get_bits(0b0101_1001), result);
    }

}