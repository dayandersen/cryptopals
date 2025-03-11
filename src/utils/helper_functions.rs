use core::panic;



// Count number of bits different
pub fn hamming_distance(s1: &str, s2: &str) -> u32 {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    hamming_distance_bytes(s1_bytes, s2_bytes)
}

// Count number of bits different
pub fn hamming_distance_bytes(bytes_1: &[u8], bytes_2: &[u8]) -> u32 {
    if bytes_1.len() != bytes_2.len() {
        panic!("You did the bad, give me equal length strings")
    }
    let mut dist = 0;

    for i in 0..bytes_1.len() {
        for shift in 0..7 {
            if bytes_1[i] >> shift & 1 != bytes_2[i] >> shift & 1 {
                dist += 1
            }
            
        }
    }
    dist
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_test() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }    
}