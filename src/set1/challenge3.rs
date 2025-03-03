use std::collections::HashMap;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::str;

use crate::utils::helper_functions;



pub fn single_char_xor_decryption(inp: &str) -> String{
    let bytes = helper_functions::hex_str_to_bytes(inp);
    let mut options: Vec<Vec<u8>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push(byte ^ i);
        }
        options.push(curr)
    }
    let mut likely_word = Vec::new() ;
    let mut likely_word_score = 0;
    for option in options {
        let curr_word_score = generate_string_score(&option);

        if curr_word_score  > likely_word_score {
            likely_word = option;
            likely_word_score = curr_word_score;
        }
    }

    return match  str::from_utf8(&likely_word) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

pub fn single_char_xor_decryption_char_array(inp: &[char]) -> String {
    let bytes = helper_functions::hex_char_array_to_bytes(inp);
    let mut options: Vec<Vec<u8>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push(byte ^ i);
        }
        options.push(curr)
    }
    let mut likely_word = Vec::new() ;
    let mut likely_word_score = 0;
    for option in options {
        let curr_word_score = generate_string_score(&option);

        if curr_word_score  > likely_word_score {
            likely_word = option;
            likely_word_score = curr_word_score;
        }
    }

    return match  str::from_utf8(&likely_word) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

pub fn generate_string_score(inp: &Vec<u8>) -> u32 {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    println!("Generating scores");
    return match str::from_utf8(inp) {
        Ok(v) => {
            // Need to see if current string has a lot of repeated characters
            
            let mut bytes_to_counts:HashMap<u8, u32> = HashMap::new();
            for byte in inp {
                *bytes_to_counts.entry(*byte).or_insert(0) += 1;
            }
            let n_characters = v.len();
            let mut vec: Vec<(&u8, &u32)> = bytes_to_counts.iter().collect::<Vec<(&u8,&u32)>>();
            vec.sort_by(|a,b| b.1.cmp(&a.1));
            let mut space_count = 0;
            let mut vowel_count = 0;
            for tup in vec {
                match *tup.0 as char {
                    ' ' => space_count += 1,
                    'a'| 'e'| 'i'| 'o'| 'u'| 'y' => vowel_count += 1,
                    _ => println!("Found garbage")
                }
                println!("tup.0: {}", *tup.0 as char);
                println!("tup.1: {}", tup.1);
            }

            if space_count == 0 {
                return 0;
            }
            if vowel_count == 0 {
                return 0;
            }
            
            if space_count + vowel_count == n_characters {
                return 0;
            }

            return 1;
        },
        Err(e) => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn do_it() {
        assert_eq!(single_char_xor_decryption("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"), "Cooking MC's like a pound of bacon");
        
    }
}