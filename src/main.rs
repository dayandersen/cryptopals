mod set1;
use set1::challenge1::hexStringToBase64;

use std::collections::HashMap;

const HEX_INPUT_VAL: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const BASE_64_OUTPUT_VAL: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    
let INT_TO_BASE64_MAPPING: HashMap<u32, char> = HashMap::from([
    (0 , 'A'),
    (1 , 'B'),
    (2 , 'C'),
    (3 , 'D'),
    (4 , 'E'),
    (5 , 'F'),
    (6 , 'G'),
    (7 , 'H'),
    (8 , 'I'),
    (9 , 'J'),
    (10, 'K'),
    (11, 'L'),
    (12, 'M'),
    (13, 'N'),
    (14, 'O'),
    (15, 'P'),
    (16, 'Q'),
    (17, 'R'),
    (18, 'S'),
    (19, 'T'),
    (20, 'U'),
    (21, 'V'),
    (22, 'W'),
    (23, 'X'),
    (24, 'Y'),
    (25, 'Z'),
    (26, 'a'),
    (27, 'b'),
    (28, 'c'),
    (29, 'd'),
    (30, 'e'),
    (31, 'f'),
    (32, 'g'),
    (33, 'h'),
    (34, 'i'),
    (35, 'j'),
    (36, 'k'),
    (37, 'l'),
    (38, 'm'),
    (39, 'n'),
    (40, 'o'),
    (41, 'p'),
    (42, 'q'),
    (43, 'r'),
    (44, 's'),
    (45, 't'),
    (46, 'u'),
    (47, 'v'),
    (48, 'w'),
    (49, 'x'),
    (50, 'y'),
    (51, 'z'),
    (52, '0'),
    (53, '1'),
    (54, '2'),
    (55, '3'),
    (56, '4'),
    (57, '5'),
    (58, '6'),
    (59, '7'),
    (60, '8'),
    (61, '9'),
    (62, '+'),
    (63, '/')
]);
// 63 	111111 	/

let HEX_CHAR_TO_DECIMAL_MAPPING: HashMap<char, u32> = HashMap::from([
    ('0', 0),
    ('1', 1 ),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('A', 10),
    ('B', 11),
    ('C', 12),
    ('D', 13),
    ('E', 14),
    ('F', 15 ),
    ('a', 10),
    ('b', 11),
    ('c', 12),
    ('d', 13),
    ('e', 14),
    ('f', 15)
]);
// 'F' 	1111 	17 	15
    
println!("{}", hexStringToBase64(HEX_INPUT_VAL, INT_TO_BASE64_MAPPING, HEX_CHAR_TO_DECIMAL_MAPPING))
}
