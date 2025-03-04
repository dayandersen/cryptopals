use std::collections::HashMap;
use std::str;
use std::ascii;

use crate::utils::helper_functions;



pub fn single_char_xor_decryption(inp: &str) -> String{
    let bytes = helper_functions::hex_str_to_bytes(inp);
    return single_char_xor_decryption_helper(bytes);
}

pub fn single_char_xor_decryption_char_array(inp: &[char]) -> String {
    let bytes = helper_functions::hex_char_array_to_bytes(inp);
    return single_char_xor_decryption_helper(bytes);
}


fn single_char_xor_decryption_helper(bytes: Vec<u8>) -> String {
    let mut options: Vec<Vec<u8>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push(byte ^ i);
        }
        options.push(curr)
    }
    let mut likely_word: &str = "";
    let mut likely_word_score = 1;
    let mut likelies: Vec<&str> = Vec::new();
    for option in &options {
        let curr_word: &str;
        match str::from_utf8(option) {
            Ok(v) => curr_word = v,
            Err(e) => continue

        }
        let curr_word_score = generate_string_score(curr_word);

        if curr_word_score  >= likely_word_score {
            likelies.push(curr_word);
            likely_word = curr_word;
            likely_word_score = curr_word_score;
        }
    }

    for s in likelies {
        println!("An option: {}", s)
    }

    return likely_word.to_string();
}

pub fn generate_string_score(inp: &str) -> u32 {
    // Need to see if current string has a lot of repeated characters
    let vowels = ['a', 'e', 'i', 'o', 'u','y'];
    let bad_chars = [ '`', '/'];

    // let mut chars_to_counts:HashMap<char, u32> = HashMap::new();
    let mut space_count = 0;
    let mut vowel_count = 0;
    let mut non_vowel_spaces = 0;
    let mut bad_char_count = 0;
    for c in inp.chars() {
        match c {
            // c if !c.is_ascii() => bad_char_count += 1,
            ' ' => space_count += 1,
            c if c.is_numeric() => bad_char_count += 1,
            c if bad_chars.contains(&c) => bad_char_count += 1,
            c if vowels.contains(&c) => vowel_count +=  1,
            _ => non_vowel_spaces += 1
        }
        // *chars_to_counts.entry(c).or_insert(0) += 1;
    }
    let n_characters = inp.len() as u32;
    // let vec: Vec<(&char, &u32)> = chars_to_counts.iter().collect::<Vec<(&char,&u32)>>();
    // vec.sort_by(|a,b| b.1.cmp(&a.1));
    
    if bad_char_count > 0 || space_count == 0 || vowel_count == 0 || non_vowel_spaces == 0 {
        return 0;
    }
    
    if space_count + vowel_count == n_characters {
        return 0;
    }

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn do_it() {
        assert_eq!(single_char_xor_decryption("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"), "Cooking MC's like a pound of bacon");   
    }

    #[test]
    pub fn generate_string_score_test() {
        assert_eq!(generate_string_score("<"), 0);
        assert_eq!(generate_string_score(" "), 0);
        assert_eq!(generate_string_score("\\a"), 0);
        assert_eq!(generate_string_score("1\\a3"), 0);
        assert_eq!(generate_string_score("o o o o o o o o o o o"), 0);
        assert_eq!(generate_string_score("CookingMC'slikeapoundofbacon"), 0);
        assert_eq!(generate_string_score("Cooking MC's like a pound of bacon"), 1);
    }
}