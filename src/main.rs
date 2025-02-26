mod set1;
mod utils;
use set1::challenge1::hex_string_to_base64;

use std::collections::HashMap;

const HEX_INPUT_VAL: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const BASE_64_OUTPUT_VAL: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    
println!("{}", hex_string_to_base64(HEX_INPUT_VAL))
}
