use std::collections::HashMap;


//Turn hex into bytes, each entry will be a u4
//Combine 2 u4s in a u8

pub fn hexStringToBase64(inp: &str, intToBase64Mapping: HashMap<u32, char>, hexCharToDecimalMapping: HashMap<char, u32>) -> String{
    let mut vec = Vec::new();
    for c in inp.chars() {
        match hexCharToDecimalMapping.get(&c) {
            //Decimal here will be a value from 00000000 to 00001111 
            //Need to translate to a value from 00000000 to 00111111
            Some(decimcal) => {
                println!("decimal is {}", *decimcal);
                vec.push(*decimcal);
            },
            None => println!("Mapping not found, da fuq")
        }
    }
    let smooshedBytes = vec.windows(2)
    .map(|d| d[0] << 4 | d[1])
    .collect::<Vec<_>>();

    // We have bytes now
    // Need to extract sets of 6 from this
    // Could do this by grabbing leftmost 111111, but things get off

    // 24 bytes , need to grab six chunks
    let mut ret: String = String::from("");
    
    for entries in smooshedBytes.windows(3) {
        let thing = entries[0] << 16 | entries[1] << 8 | entries[2];
        let key1 = thing >> 18 & 63;
        let key2 = thing >> 12 & 63;
        let key3 = thing >> 6 & 63;
        let key4 = thing & 63;
        println!("thing is {}", thing);
        
        println!("Trying to access keys: {}, {}, {}, {}", key1, key2, key3, key4);
        
        ret.push_str(&format!("{}{}{}{}",
            intToBase64Mapping[&key1],
            intToBase64Mapping[&key2],
            intToBase64Mapping[&key3],
            intToBase64Mapping[&key4]))
    }
    println!("{}",ret);
    return ret
}

#[cfg(test)]
mod tests {
    const HEX_INPUT_VAL: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const BASE_64_OUTPUT_VAL: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    use super::*;

    #[test]
    fn it_works() {
        let int_to_base64_mapping: HashMap<u32, char> = HashMap::from([
            (0, 'A'), (1, 'B'), (2, 'C'), (3, 'D'), (4, 'E'), (5, 'F'),
            (6, 'G'), (7, 'H'), (8, 'I'), (9, 'J'), (10, 'K'), (11, 'L'),
            (12, 'M'), (13, 'N'), (14, 'O'), (15, 'P'), (16, 'Q'), (17, 'R'),
            (18, 'S'), (19, 'T'), (20, 'U'), (21, 'V'), (22, 'W'), (23, 'X'),
            (24, 'Y'), (25, 'Z'), (26, 'a'), (27, 'b'), (28, 'c'), (29, 'd'),
            (30, 'e'), (31, 'f'), (32, 'g'), (33, 'h'), (34, 'i'), (35, 'j'),
            (36, 'k'), (37, 'l'), (38, 'm'), (39, 'n'), (40, 'o'), (41, 'p'),
            (42, 'q'), (43, 'r'), (44, 's'), (45, 't'), (46, 'u'), (47, 'v'),
            (48, 'w'), (49, 'x'), (50, 'y'), (51, 'z'), (52, '0'), (53, '1'),
            (54, '2'), (55, '3'), (56, '4'), (57, '5'), (58, '6'), (59, '7'),
            (60, '8'), (61, '9'), (62, '+'), (63, '/')
        ]);

        let hex_char_to_decimal_mapping: HashMap<char, u32> = HashMap::from([
            ('0', 0), ('1', 1), ('2', 2), ('3', 3), ('4', 4), ('5', 5),
            ('6', 6), ('7', 7), ('8', 8), ('9', 9), ('A', 10), ('B', 11),
            ('C', 12), ('D', 13), ('E', 14), ('F', 15), ('a', 10), ('b', 11),
            ('c', 12), ('d', 13), ('e', 14), ('f', 15)
        ]);

        let result = hexStringToBase64(
            HEX_INPUT_VAL,
            int_to_base64_mapping,
            hex_char_to_decimal_mapping
        );
        
        assert_eq!(result, BASE_64_OUTPUT_VAL, 
            "Expected '{}' but got '{}'", BASE_64_OUTPUT_VAL, result);
    }

    #[test]
    fn test_empty_string() {
        let int_to_base64_mapping: HashMap<u32, char> = HashMap::new();
        let hex_char_to_decimal_mapping: HashMap<char, u32> = HashMap::new();
        let result = hexStringToBase64("", int_to_base64_mapping, hex_char_to_decimal_mapping);
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_byte() {
        let int_to_base64_mapping: HashMap<u32, char> = HashMap::from([
            (0, 'A'), (16, 'Q'), (32, 'g')
        ]);
        let hex_char_to_decimal_mapping: HashMap<char, u32> = HashMap::from([
            ('4', 4), ('1', 1)
        ]);
        let result = hexStringToBase64("41", int_to_base64_mapping, hex_char_to_decimal_mapping);
        assert_eq!(result, "QQ");
    }
}