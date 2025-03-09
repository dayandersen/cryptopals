use crate::{set1::challenge3::single_char_xor_decryption_xor_key, utils::*};

pub fn turning_up_the_heat() {
    // INPUT_STR
    //try values from 2 to (say) 40. 
    // base64::base64_char_to_bytes(c);
    
    let inp_str_bytes = base64::base64_str_to_bytes_str(INPUT_STR.replace('\n', ""));
    // let distances_and_key_size = normalized_edit_distances_to_key_size(inp_str_bytes);
    let distance_and_key_size_four = normalized_edit_distances_to_key_size_four_v(inp_str_bytes.clone());

    for distance_to_key_size_tup in &distance_and_key_size_four[0..10] {
        println!("Generating XORs for keysize: {}", distance_to_key_size_tup.1);
        let key_size = distance_to_key_size_tup.1;
        let blocks = generate_byte_blocks_v(inp_str_bytes.clone(), key_size as usize);

        for block in blocks {
            print!("{}", single_char_xor_decryption_xor_key(block) as char)
        }
        println!();
    }
}


fn normalized_edit_distances_to_key_size(inp_bytes: &[u8]) -> Vec<(u32,u32)>{
    let mut distances_and_key_size: Vec<(u32, u32)> = Vec::new();
    
    for key_size in 2..40 {

        let mut distances:Vec<u32> = Vec::new();

        for i in (0..160/key_size).step_by(key_size*2) {
            distances.push(helper_functions::hamming_distance_bytes(
                &inp_bytes[i.. i +key_size], 
                &inp_bytes[i+key_size..i + key_size * 2]
            ) * 1000)
        }
        
        distances_and_key_size.push(((distances.iter().sum::<u32>())/(key_size * distances.len()) as u32 , key_size as u32));
    }

    distances_and_key_size.sort();
    return distances_and_key_size;
}

fn normalized_edit_distances_to_key_size_four(inp_bytes: &[u8]) -> Vec<(u32,u32)>{
    return normalized_edit_distances_to_key_size_four_v(inp_bytes.to_vec())
}


fn normalized_edit_distances_to_key_size_four_v(inp_bytes: Vec<u8>) -> Vec<(u32,u32)>{
    let mut distances_and_key_size: Vec<(u32, u32)> = Vec::new();
    
    for key_size in 2..40 {

        let mut distances:Vec<u32> = Vec::new();

        distances.push(helper_functions::hamming_distance_bytes(
            &inp_bytes[0..key_size], 
            &inp_bytes[key_size..key_size*2]
        )* 1000);
        distances.push(helper_functions::hamming_distance_bytes(
            &inp_bytes[key_size*2..key_size*3], 
            &inp_bytes[key_size*3..key_size*4]
        )* 1000);
        
        distances_and_key_size.push(((distances.iter().sum::<u32>())/ (key_size * distances.len()) as u32 , key_size as u32));
    }

    distances_and_key_size.sort();
    return distances_and_key_size;
}

fn generate_byte_blocks(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    return generate_byte_blocks_v(bytes.to_vec(), key_size)
}

fn generate_byte_blocks_v(bytes: Vec<u8>, key_size: usize) -> Vec<Vec<u8>> {
    let mut blocks:Vec<Vec<u8>> = Vec::new();
    for _ in 0..key_size {
        blocks.push(Vec::new());
    }
    // break the ciphertext into blocks of KEYSIZE length. 
    for chunk in bytes.chunks(key_size) {
        // make a block that is the first byte of every block,
        // and a block that is the second byte of every block, and so on. 
        let mut count = 0;
        for byte in chunk {
            blocks[count % key_size].push(*byte);
            count += 1
        }
    }
    return blocks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_work() {
        turning_up_the_heat()
    }

    #[test]
    pub fn generate_byte_blocks_test() {
        let bytes = [1,2,3,4,5];
        let res = generate_byte_blocks(&bytes, 2);
        assert_eq!(res[0], Vec::from([0b1,0b11,0b101]));
        assert_eq!(res[1], Vec::from([0b10,0b100]));


        let bytes = [1,2,3,4,5,6,7,8,9];
        let res = generate_byte_blocks(&bytes, 3);
        assert_eq!(res[0], Vec::from([1,4,7]));
        assert_eq!(res[1], Vec::from([2,5,8]));
        assert_eq!(res[2], Vec::from([3,6,9]));
    }

    #[test]
    pub fn normalized_edit_distances_to_key_size_test() {
        let key_distances_four = normalized_edit_distances_to_key_size_four(INPUT_STR.as_bytes());
        let key_distances = normalized_edit_distances_to_key_size(INPUT_STR.as_bytes());

    }
}


const INPUT_STR: &str = "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";
