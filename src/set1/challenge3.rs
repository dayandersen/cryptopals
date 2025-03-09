use std::collections::HashMap;
use std::str;

use crate::utils::*;


pub struct XorScoreResp {
    word: String,
    score: f32,
    xor_key: u8,
}

pub fn single_char_xor_decryption(inp: &str) -> String{
    let bytes = hex::hex_str_to_bytes(inp);
    return single_char_xor_decryption_helper(bytes).word;
}

pub fn single_char_xor_decryption_char_array(inp: &[char]) -> String {
    let bytes = hex::hex_char_array_to_bytes(inp);
    return single_char_xor_decryption_helper(bytes).word;
}

pub fn single_char_xor_decryption_xor_key(bytes: Vec<u8>) -> u8 {
    single_char_xor_decryption_helper(bytes).xor_key
}


pub fn single_char_xor_decryption_helper(bytes: Vec<u8>) -> XorScoreResp {
    let mut likely_word: String = "".to_string();
    let mut likely_word_score = f32::MAX;
    let mut best_xor: u8 = 0;
    for xor_key in 0..=u8::MAX as u8 {
        let mut curr = Vec::new();
        for byte in &bytes {
            curr.push(byte ^ xor_key);
        }

        let curr_word = match str::from_utf8(&curr) {
            Ok(v) => v,
            Err(e) => continue
        };
        let curr_word_score = generate_string_score(curr_word);
    
        if curr_word_score <= likely_word_score {
            likely_word = curr_word.to_string();
            likely_word_score = curr_word_score;
            best_xor = xor_key;
        }
    }

    return XorScoreResp {
        word: likely_word,
        score: likely_word_score,
        xor_key: best_xor,
    }
}

pub fn generate_string_score(inp: &str) -> f32 {
    let mut frequency_map: HashMap<char, f32> = HashMap::new();
    
    for &(c, freq) in CHARS_TO_FREQUENCY {
        frequency_map.insert(c, freq / 100.0);
    }

    let (chars_to_counts, num_valid_chars) = generate_char_counts(inp, &frequency_map);

    let char_frequencies = generate_frequency_map(chars_to_counts, num_valid_chars);

    let mut score = 0.0;

    if char_frequencies.keys().len() == 0 {
        return f32::MAX;
    }

    for tup in char_frequencies {
        if !frequency_map.contains_key(&tup.0) {
            return f32::MAX;
        }
        // TODO: I should probably normalize the char CHARS_TO_FREQUENCY map to sum to 1, not 100
        score += (frequency_map[&tup.0] - tup.1).abs();
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
        let mut frequency_map: HashMap<char, f32> = HashMap::new();
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
    (' ', 15.15),
    ('E', 9.6),
    ('T', 7.2),
    ('A', 6.5),
    ('O', 6.0),
    ('I', 5.7),
    ('N', 5.5),
    ('S', 5.0),
    ('R', 4.7),
    ('H', 4.7),
    ('D', 3.4),
    ('L', 3.1),
    ('C', 2.1),
    ('U', 2.2),
    ('M', 2.0),
    ('W', 1.6),
    ('F', 1.8),
    ('G', 1.5),
    ('Y', 1.6),
    ('P', 1.3),
    ('B', 1.1),
    ('V', 0.8),
    ('K', 0.5),
    ('J', 0.08),
    ('X', 0.13),
    ('Q', 0.08),
    ('Z', 0.05),
    ('\n', 1.2),
    ('\t', 0.3),
    ('\r', 0.1),
    ('.', 0.7),
    (',', 0.45),
    ('!', 0.04),
    ('?', 0.07),
    ('\'', 0.35),
    ('"', 0.15),
    ('<', 0.01),
    ('>', 0.01),
    ('(', 0.2),
    ('[', 0.04),
    ('{', 0.02),
    ('@', 0.06),
    ('\\', 0.01),
    ('#', 0.05),
    ('$', 0.04),
    ('%', 0.04),
    ('^', 0.01),
    ('&', 0.04),
    ('*', 0.04),
    ('-', 0.3),
    ('+', 0.04),
    ('_', 0.05),
    ('=', 0.04),
    ('|', 0.01),
    ('/', 0.1),
    (';', 0.07),
    (':', 0.1),
    ('0', 0.28),
    ('1', 0.38),
    ('2', 0.24),
    ('3', 0.19),
    ('4', 0.14),
    ('5', 0.14),
    ('6', 0.11),
    ('7', 0.11),
    ('8', 0.11),
    ('9', 0.11),
    ('~', 0.01),
    ('`', 0.01),
    ('°', 0.01),
    ('±', 0.01),
    ('•', 0.02),
    ('…', 0.03),
    ('£', 0.01),
    ('€', 0.01),
    ('¥', 0.005),
    ('©', 0.01),
    ('®', 0.01),
    ('™', 0.01),
    ('§', 0.005),
    ('¶', 0.005),
];