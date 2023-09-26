use super::bit_vector_iterator::BitVectorIterator;
use super::integer_codes::IntegerCode;
use crate::introduction::bit_vector::BitVector;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn check<C: IntegerCode<u64>, P: AsRef<Path>>(
    compressed_filename: P,
    uncompressed_filename: P,
) {
    let mut bits = BitVector::new();
    let file = File::open(compressed_filename).expect("Failed to open compressed file");
    let reader = BufReader::new(file);
    bits.load(reader);

    let mut it = BitVectorIterator::new(&bits, 0);
    let num_lists = it.take(32);

    let file = File::open(uncompressed_filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    println!("checking {} lists...", num_lists);
    let mut list_size: u64 = 0;
    let mut expected: u64 = 0;
    let mut prev_x: u64 = 0;
    let mut x: u64 = 0;

    reader.lines().for_each(|line| {
        if list_size == 0 {
            list_size = it.take(32);
            expected = line.unwrap().parse().unwrap();
            if list_size != expected {
                panic!("expected list_size {} but got {}", expected, list_size);
            }
            prev_x = 0;
            x = 0;
        } else {
            x = C::read(&mut it) + prev_x;
            assert!(x >= prev_x);
            expected = line.unwrap().parse().unwrap();
            if x != expected {
                panic!("expected {} but got {}", expected, x);
            }
            prev_x = x;
            list_size -= 1;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::super::integer_codes::{DeltaCode, GammaCode, RiceCode, VByteCode};
    use super::*;

    #[test]
    fn check_gamma() {
        check::<GammaCode, _>("data/out_gamma.bin", "data/lists.txt");
    }

    #[test]
    fn check_delta() {
        check::<DeltaCode, _>("data/out_delta.bin", "data/lists.txt");
    }

    #[test]
    fn check_vbyte() {
        check::<VByteCode, _>("data/out_vbyte.bin", "data/lists.txt");
    }

    #[test]
    fn check_rice_k1() {
        check::<RiceCode<1>, _>("data/out_rice_k1.bin", "data/lists.txt");
    }

    #[test]
    fn check_rice_k2() {
        check::<RiceCode<2>, _>("data/out_rice_k2.bin", "data/lists.txt");
    }
}
