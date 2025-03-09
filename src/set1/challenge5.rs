use crate::utils::*;



pub fn ice_ice_baby() -> String {
    let xor_target = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let xor = "ICE".as_bytes();
    let mut count = 0;
    let mut xored: Vec<u8> = Vec::new();
    for byte in xor_target.as_bytes() {
        xored.push(byte ^ xor[count % 3]);
        count += 1
    }
    return hex::bytes_to_hex_str(xored);
}

#[cfg(test)]
mod tests {
    use super::ice_ice_baby;

    #[test]
    pub fn do_it() {
        assert_eq!(ice_ice_baby(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}