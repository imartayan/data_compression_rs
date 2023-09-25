use super::util::num_64bit_words_for;
use std::mem::swap;

pub struct BitVector {
    num_bits: usize,
    bits: Vec<u64>,
}

impl BitVector {
    pub fn new() -> Self {
        Self {
            num_bits: 0,
            bits: Vec::new(),
        }
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits
    }

    pub fn num_64bit_words(&self) -> usize {
        self.bits.len()
    }

    pub fn get_bits(&self, pos: usize, len: usize) -> u64 {
        assert!(pos + len <= self.num_bits);
        assert!(len <= 64);

        if len == 0 {
            return 0;
        }

        let block = pos / 64;
        let shift = pos % 64;
        let mask = (1u64 << len).wrapping_sub(1);

        if shift + len <= 64 {
            (self.bits[block] >> shift) & mask
        } else {
            (self.bits[block] >> shift) | ((self.bits[block + 1] << (64 - shift)) & mask)
        }
    }

    pub fn get_word64(&self, pos: usize) -> u64 {
        assert!(pos <= self.num_bits);

        let block = pos / 64;
        let shift = pos % 64;
        let mut word = self.bits[block] >> shift;

        if (shift > 0) && (block + 1 < self.bits.len()) {
            word |= self.bits[block + 1] << (64 - shift);
        }

        word
    }
}

pub struct BitVectorBuilder {
    num_bits: usize,
    bits: Vec<u64>,
}

impl BitVectorBuilder {
    pub fn new() -> Self {
        Self {
            num_bits: 0,
            bits: Vec::new(),
        }
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits
    }

    pub fn resize(&mut self, num_bits: usize) {
        self.num_bits = num_bits;
        self.bits.resize(num_64bit_words_for(num_bits), 0);
    }

    pub fn reserve(&mut self, num_bits: usize) {
        self.bits.reserve(num_64bit_words_for(num_bits));
    }

    pub fn build(&mut self, bv: &mut BitVector) {
        swap(&mut self.num_bits, &mut bv.num_bits);
        bv.bits.resize(self.bits.len(), 0);
        self.bits.swap_with_slice(&mut bv.bits);
    }

    pub fn set_bits(&mut self, pos: usize, x: u64, len: usize) {
        assert!(pos + len <= self.num_bits);
        assert!(len == 64 || (x >> len) == 0); // no other bits must be set
        if len == 0 {
            return;
        }

        let mask = (1u64 << len).wrapping_sub(1);
        let word = pos / 64;
        let pos_in_word = pos % 64;

        self.bits[word] &= !(mask << pos_in_word);
        self.bits[word] |= x << pos_in_word;

        let stored = 64 - pos_in_word;
        if stored < len {
            self.bits[word + 1] &= !(mask >> stored);
            self.bits[word + 1] |= x >> stored;
        }
    }

    pub fn append_bits(&mut self, x: u64, len: usize) {
        assert!(len <= 64);
        assert!(len == 64 || (x >> len) == 0); // no other bits must be set
        if len == 0 {
            return;
        }

        let pos_in_word = self.num_bits % 64;
        self.num_bits += len;
        if pos_in_word == 0 {
            self.bits.push(x);
        } else {
            let cur_word = self.bits.last_mut().unwrap();
            *cur_word |= x << pos_in_word;
            if len > 64 - pos_in_word {
                self.bits.push(x >> (64 - pos_in_word));
            }
        }
    }
}
