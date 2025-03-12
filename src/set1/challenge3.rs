use core::str;
use std::collections::HashMap;

use crate::utils::hex;


pub struct XorScoreResp {
    pub word: String,
    pub score: f32,
    pub xor_key: u8,
}

pub fn single_char_xor_decryption(inp: &str) -> String {
    single_char_xor_decryption_helper(&hex::hex_str_to_bytes(inp)).word
}

pub fn single_char_xor_decryption_xor_key(bytes: &[u8]) -> u8 {
    single_char_xor_decryption_helper(bytes).xor_key
}

pub fn single_char_xor_decryption_helper(bytes: &[u8]) -> XorScoreResp {
    let mut likely_word: String = String::new();
    let mut likely_word_score = f32::MAX;
    let mut best_xor: u8 = 0;
    for xor_key in 0..=u8::MAX {
        let curr =&bytes.iter().map(|byte| byte ^ xor_key).collect::<Vec<u8>>();
        let Ok(curr_word) = str::from_utf8(curr) else { continue;};
        let curr_word_score = generate_string_score(curr_word);
        if curr_word_score < likely_word_score {
            likely_word = curr_word.to_string();
            likely_word_score = curr_word_score;
            best_xor = xor_key;
        }
    }

    XorScoreResp {
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
    let char_count:f32 = char_frequencies.values().sum();
    if char_frequencies.keys().len() == 0 {
        return f32::MAX;
    }

    for tup in char_frequencies {
        if !frequency_map.contains_key(&tup.0) {
            return f32::MAX;
        }
        score += (frequency_map[&tup.0] - tup.1).abs();
    }
    
    // Division by 0 handled by key check. If there is at least 1 key there will be at least 1 value
    score / char_count
}


fn generate_frequency_map(chars_to_count: HashMap<char, f32>, total_num: f32) -> HashMap<char, f32> {
    let mut frequencies:HashMap<char, f32> = HashMap::new();

    for tup in chars_to_count {
        frequencies.insert(tup.0, tup.1 / total_num);
    }

    frequencies
}

fn generate_char_counts(inp: &str, allowable_chars:&HashMap<char, f32>) -> (HashMap<char, f32>, f32) {
    let pair_chars = ['[', ']', '{','}','(',')'];
    let mut num_valid_chars = 0.0;

    let mut chars_to_counts:HashMap<char, f32> = HashMap::new();
    for c in inp.chars() {
        num_valid_chars += 1.0;
        // We're treating pair_chars as just their first component
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
        else if allowable_chars.contains_key(&c) || c.is_ascii_lowercase() {
            *chars_to_counts.entry(c.to_ascii_uppercase()).or_insert(0.0) += 1.0; 
        }
        else {
            num_valid_chars -= 1.0;
        }
    }
    (chars_to_counts, num_valid_chars)

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn generate_char_counts_test() {
        let mut frequency_map: HashMap<char, f32> = HashMap::new();

        for tup in CHARS_TO_FREQUENCY {
            frequency_map.insert(tup.0, tup.1);
        }

        let val_1 = "CookingMC'slikeapoundofbacon";
        let val_2 = "Cooking MC's like a pound of bacon";
        let (val_1_char_counts, char_count_1) = generate_char_counts(val_1, &frequency_map);
        let (val_2_char_counts, char_count_2) = generate_char_counts(val_2, &frequency_map);

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
        assert!(generate_string_score("CookingMC'slikeapoundofbacon") > generate_string_score("Cooking MC's like a pound of bacon"));
        assert_eq!(generate_string_score("aa"), generate_string_score("aaa"));
    }
}