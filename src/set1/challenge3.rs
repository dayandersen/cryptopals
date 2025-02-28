use std::fs::OpenOptions;

use crate::utils::helper_functions;



pub fn single_char_xor_decryption(inp: &str) {
    let bytes = helper_functions::hex_str_to_bytes(inp);
    let mut options: Vec<Vec<char>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push((byte ^ i) as char);
        }
        options.push(curr)
    }
    
    for option in options {
        for c in option {
            print!("{}", c)
        }
        println!()
    }
}

pub fn single_char_xor_decryption_char_array(inp: &[char]) {
    let bytes = helper_functions::hex_char_array_to_bytes(inp);
    let mut options: Vec<Vec<char>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push((byte ^ i) as char);
        }
        options.push(curr)
    }
    
    for option in options {
        for c in option {
            print!("{}", c)
        }
        println!()
    }
}

pub fn generate_string_score(inp: Vec<u8>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn do_it() {
        single_char_xor_decryption("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
    }
}