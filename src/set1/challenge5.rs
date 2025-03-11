use crate::utils::*;

pub fn xor_string(xor_key: &str, string: &str) -> String {
    
    let mut xored: Vec<u8> = Vec::new();
    for (count, byte) in string.as_bytes().iter().enumerate() {
        xored.push(byte ^ xor_key.as_bytes()[count % 3]);
    }
    hex::bytes_to_hex_str(&xored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn do_it() {
        assert_eq!(xor_string("ICE", "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"), 
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}