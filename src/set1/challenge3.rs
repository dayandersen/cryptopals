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
        else if allowable_chars.contains_key(&c){
            *chars_to_counts.entry(c).or_insert(0.0) += 1.0; 
        }
        else {
            num_valid_chars -= 1.0;
        }
    }
    (chars_to_counts, num_valid_chars)

}

const CHARS_TO_FREQUENCY:&[(char, f32)] = &[
    (' ', 15.0),
    ('e', 8.12),
    ('t', 5.92),
    ('a', 5.53),
    ('o', 5.14),
    ('i', 4.94),
    ('n', 4.63),
    ('s', 4.14),
    ('r', 3.87),
    ('h', 3.83),
    ('l', 2.70),
    ('d', 2.43),
    ('c', 2.13),
    ('u', 1.94),
    ('m', 1.62),
    ('w', 1.57),
    ('f', 1.54),
    ('g', 1.33),
    ('y', 1.30),
    ('p', 1.29),
    ('b', 1.10),
    ('v', 0.88),
    ('k', 0.52),
    ('j', 0.13),
    ('x', 0.13),
    ('q', 0.11),
    ('z', 0.07),
    ('E', 0.65),
    ('T', 0.49),
    ('A', 0.44),
    ('O', 0.41),
    ('I', 0.38),
    ('N', 0.37),
    ('S', 0.34),
    ('R', 0.32),
    ('H', 0.32),
    ('D', 0.23),
    ('L', 0.21),
    ('C', 0.14),
    ('U', 0.15),
    ('M', 0.14),
    ('W', 0.11),
    ('F', 0.12),
    ('G', 0.10),
    ('Y', 0.11),
    ('P', 0.09),
    ('B', 0.07),
    ('V', 0.05),
    ('K', 0.03),
    ('J', 0.01),
    ('X', 0.01),
    ('Q', 0.01),
    ('Z', 0.003),
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
    (')', 0.2),
    ('[', 0.04),
    (']', 0.04),
    ('{', 0.02),
    ('}', 0.02),
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
        assert_eq!(*val_1_char_counts.get(&'o').unwrap(), 5.0);
        assert_eq!(*val_2_char_counts.get(&' ').unwrap(), 6.0);
        assert_eq!(*val_2_char_counts.get(&'o').unwrap(), 5.0);
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