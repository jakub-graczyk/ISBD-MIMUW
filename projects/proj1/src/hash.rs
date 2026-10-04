pub fn sum_block_hash(result: &mut Vec<u8>, block_hash: &[u8; 16]) {
    for i in 0..16 {
        result[i] ^= block_hash[i];
    }
}