/// number of 64-bit words for num_bits
pub fn num_64bit_words_for(num_bits: usize) -> usize {
    (num_bits + 64 - 1) / 64
}
