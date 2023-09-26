use super::integer_codes::IntegerCode;
use crate::integer_codes::bit_vector_iterator::BitVectorIterator;
use crate::introduction::bit_vector::BitVector;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

pub fn decompress<C: IntegerCode<u64>, P: AsRef<Path>>(input_filename: P) {
    let mut bits = BitVector::new();
    let file = File::open(input_filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    bits.load(reader);

    println!("loaded {} bits", bits.num_bits());

    let mut it = BitVectorIterator::new(&bits, 0);
    let num_lists = it.take(32);
    let mut num_ints = 0;

    println!("decompressing {} lists...", num_lists);
    let mut sum = 0;
    let now = Instant::now();

    for _ in 0..num_lists {
        let list_size = it.take(32);
        let mut prev_x = 0;
        let mut x;
        for _ in 0..list_size {
            x = C::read(&mut it) + prev_x;
            assert!(x >= prev_x);
            prev_x = x;
            sum += x;
        }
        num_ints += list_size;
    }

    let elapsed = now.elapsed().as_micros();
    println!("(ignore: {})", sum);
    println!("decompressed {} integers in {} µs", num_ints, elapsed);
    println!("({:.2} ns/int)", elapsed as f64 * 1000.0 / num_ints as f64);
}

#[cfg(test)]
mod tests {
    use super::super::integer_codes::{DeltaCode, GammaCode, RiceCode, VByteCode};
    use super::*;

    #[test]
    fn decompress_gamma() {
        decompress::<GammaCode, _>("data/out_gamma.bin");
    }

    #[test]
    fn decompress_delta() {
        decompress::<DeltaCode, _>("data/out_delta.bin");
    }

    #[test]
    fn decompress_vbyte() {
        decompress::<VByteCode, _>("data/out_vbyte.bin");
    }

    #[test]
    fn decompress_rice_k1() {
        decompress::<RiceCode<1>, _>("data/out_rice_k1.bin");
    }

    #[test]
    fn decompress_rice_k2() {
        decompress::<RiceCode<2>, _>("data/out_rice_k2.bin");
    }
}
