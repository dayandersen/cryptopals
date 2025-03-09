use crate::utils::*;

//Turn hex into bytes, each entry will be a u4
//Combine 2 u4s in a u8

pub fn hex_string_to_base64(inp: &str ) -> String{
    base64::bytes_to_base_64_str(false, hex::hex_str_to_bytes(inp))
}

#[cfg(test)]
mod tests {
    const HEX_INPUT_VAL: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const BASE_64_OUTPUT_VAL: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    use super::*;

    #[test]
    fn it_works() {
    
        let result = hex_string_to_base64(
            HEX_INPUT_VAL,
        );
        
        assert_eq!(result, BASE_64_OUTPUT_VAL, 
            "Expected '{}' but got '{}'", BASE_64_OUTPUT_VAL, result);
    }

    #[test]
    fn test_empty_string() {
        let result = hex_string_to_base64("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_byte() {
        let result = hex_string_to_base64("41");
        assert_eq!(result, "QQ");
    }
}