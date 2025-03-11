use crate::utils::hex;
use std::iter::zip;

pub fn xor_inputs(inp1: &str, inp2: &str) -> String {
    let bytes_1 =hex::hex_str_to_bytes(inp1);
    let bytes_2 = hex::hex_str_to_bytes(inp2);

    let mut xored: Vec<u8> = Vec::new();

    for bytes in zip(bytes_1, bytes_2) {
        xored.push(bytes.0 ^ bytes.1);
    }

    hex::bytes_to_hex_str(&xored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() { 
        
        assert_eq!(xor_inputs("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), "746865206b696420646f6e277420706c6179");
    }
}