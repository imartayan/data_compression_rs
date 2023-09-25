use num_traits::int::PrimInt;

/// number of 64-bit words for num_bits
pub fn num_64bit_words_for(num_bits: usize) -> usize {
    (num_bits + 64 - 1) / 64
}

/// position of the most significant bit (msb)
pub fn msb<T: PrimInt>(x: T) -> usize {
    let num_bits = T::zero().count_zeros();
    (num_bits - 1 - x.leading_zeros()) as usize
}

/// position of the least significant bit (lsb)
pub fn lsb<T: PrimInt>(x: T) -> usize {
    x.trailing_zeros() as usize
}
