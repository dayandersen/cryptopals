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


pub fn single_char_xor_decryption_helper(bytes: Vec<u8>) -> String {
    let mut options: Vec<Vec<u8>> = Vec::new();
    for i in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push(byte ^ i);
        }
        options.push(curr)
    }
    let mut likely_word: &str = "";
    let mut likely_word_score = 100000.0;
    let mut likelies: Vec<&str> = Vec::new();
    for option in &options {
        let curr_word: &str;
        match str::from_utf8(option) {
            Ok(v) => curr_word = v,
            Err(e) => continue

        }
        let curr_word_score = generate_string_score(curr_word);

        if curr_word_score < likely_word_score {
            likely_word = curr_word;
            likely_word_score = curr_word_score;
        }
    }

    return likely_word.to_string();
}

pub fn generate_string_score(inp: &str) -> f32 {

    let mut frequency_map: HashMap<char, f32> = HashMap::new();

    for tup in CHARS_TO_FREQUENCY {
        frequency_map.insert(tup.0, tup.1);
    }

    let (chars_to_counts, num_valid_chars) = generate_char_counts(inp, &frequency_map);
    if num_valid_chars != inp.len() as f32 {
        return 10000.0;
    }

    let char_frequencies = generate_frequency_map(chars_to_counts, num_valid_chars);

    let mut score = 0.0;
    for tup in char_frequencies {
        // TODO: I should probably normalize the char CHARS_TO_FREQUENCY map to sum to 1, not 100
        score += (frequency_map[&tup.0] - (tup.1*100.0)).abs();
    }

    return score;
}

fn generate_frequency_map(chars_to_count: HashMap<char, f32>, total_num: f32) -> HashMap<char, f32> {
    let mut frequencies:HashMap<char, f32> = HashMap::new();

    for tup in chars_to_count {
        frequencies.insert(tup.0, tup.1 / total_num);
    }

    return  frequencies;
}

fn generate_char_counts(inp: &str, frequency_map:&HashMap<char, f32>) -> (HashMap<char, f32>, f32) {
    let pair_chars = ['[', ']', '{','}','(',')'];
    let mut num_valid_chars = 0.0;

    let mut chars_to_counts:HashMap<char, f32> = HashMap::new();
    for c in inp.chars() {
        num_valid_chars += 1.0;
        // We're treating pair_chars as just their first 
        if pair_chars.contains(&c) {
            if c == '{' || c == '}' {
                *chars_to_counts.entry('{').or_insert(0.0) += 1.0;
            }
            if c == '(' || c == ')' {
                *chars_to_counts.entry('(').or_insert(0.0) += 1.0;
            }
            if c == '[' || c == ']' {
                *chars_to_counts.entry('[').or_insert(0.0) += 1.0;
            }
        }
        else if c.is_ascii_lowercase() {
            *chars_to_counts.entry(c.to_ascii_uppercase()).or_insert(0.0) += 1.0; 
        }
        else if frequency_map.contains_key(&c){
            *chars_to_counts.entry(c.to_ascii_uppercase()).or_insert(0.0) += 1.0;
        } else {
            num_valid_chars -= 1.0;
            continue;
        }
    }
    return (chars_to_counts, num_valid_chars);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn summ() {
        let mut summ = 0.0;
        for tup in CHARS_TO_FREQUENCY {
            summ += tup.1
        }
        println!("{}", summ);
    }

    #[test]
    pub fn generate_char_counts_test() {
        let mut frequency_map: HashMap<char, f32> = HashMap::new();

        for tup in CHARS_TO_FREQUENCY {
            frequency_map.insert(tup.0, tup.1);
        }

        let val_1 = "CookingMC'slikeapoundofbacon";
        let val_2 = "Cooking MC's like a pound of bacon";
        let (val_1_char_counts, char_count_1) = generate_char_counts(&val_1, &frequency_map);
        let (val_2_char_counts, char_count_2) = generate_char_counts(&val_2, &frequency_map);

        assert!(!val_1_char_counts.contains_key(&' '));
        assert_eq!(char_count_1, 28.0);
        assert_eq!(char_count_2, 34.0);
        assert_eq!(*val_1_char_counts.get(&'O').unwrap(), 5.0);
        assert_eq!(*val_2_char_counts.get(&' ').unwrap(), 6.0);
        assert_eq!(*val_2_char_counts.get(&'O').unwrap(), 5.0);
        
    }

    #[test]
    pub fn do_it() {
        assert_eq!(single_char_xor_decryption("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"), "Cooking MC's like a pound of bacon");   
    }

    #[test]
    pub fn generate_string_score_test() {
        let val_1 = "CookingMC'slikeapoundofbacon";
        let val_2 = "Cooking MC's like a pound of bacon";
        let string_score_val_1 = generate_string_score(val_1);
        let string_score_val_2 = generate_string_score(val_2);

        assert_ne!(string_score_val_1, string_score_val_2);

        println!("string_score_val_1: {string_score_val_1}");
        println!("string_score_val_2: {string_score_val_2}");

        assert!(string_score_val_1 > string_score_val_2)
    }
}
const CHARS_TO_FREQUENCY:&[(char, f32)] = &[
    (' ', 16.5),  // Adjusted to balance total
    ('E', 10.2),  // Scaled
    ('T', 7.7),   // Scaled
    ('A', 6.9),   // Scaled
    ('O', 6.5),   // Scaled
    ('I', 6.2),   // Scaled
    ('N', 5.9),   // Scaled
    ('S', 5.3),   // Scaled
    ('H', 5.0),   // Scaled
    ('R', 5.0),   // Scaled
    ('D', 3.7),   // Scaled
    ('L', 3.4),   // Scaled
    ('C', 2.3),   // Scaled
    ('U', 2.4),   // Scaled
    ('M', 2.2),   // Scaled
    ('W', 1.8),   // Scaled
    ('F', 2.0),   // Scaled
    ('G', 1.7),   // Scaled
    ('Y', 1.8),   // Scaled
    ('P', 1.5),   // Scaled
    ('B', 1.3),   // Scaled
    ('V', 0.9),   // Scaled
    ('K', 0.6),   // Scaled
    ('J', 0.1),   // Maintained
    ('X', 0.15),  // Scaled
    ('Q', 0.1),   // Maintained
    ('Z', 0.07),  // Maintained
    ('.', 0.8),   // Adjusted
    (',', 0.5),   // Adjusted
    ('!', 0.04),  // Adjusted
    ('?', 0.08),  // Adjusted
    ('\'', 0.4),  // Adjusted
    ('"', 0.18),  // Adjusted
    ('<', 0.01),  // Maintained
    ('>', 0.01),  // Maintained
    ('(', 0.24),  // Adjusted
    ('[', 0.04),  // Adjusted
    ('{', 0.02),  // Maintained
    ('@', 0.07),  // Adjusted
    ('\\', 0.01), // Maintained
    ('#', 0.06),  // Adjusted
    ('$', 0.04),  // Adjusted
    ('%', 0.04),  // Adjusted
    ('^', 0.01),  // Maintained
    ('&', 0.04),  // Adjusted
    ('*', 0.04),  // Adjusted
    ('-', 0.35),  // Adjusted
    ('+', 0.04),  // Adjusted
    ('_', 0.06),  // Adjusted
    ('=', 0.04),  // Adjusted
    ('|', 0.01),  // Maintained
    ('/', 0.12),  // Adjusted
    (';', 0.08),  // Adjusted
    (':', 0.12),  // Adjusted
];