use super::integer_codes::IntegerCode;
use crate::introduction::bit_vector::{BitVector, BitVectorBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn compress<C: IntegerCode<u64>, P: AsRef<Path>>(input_lists_filename: P, output_filename: P) {
    let mut builder = BitVectorBuilder::new();
    builder.append_bits(0, 32); // reserve the first 32-bit int for num_lists

    let mut num_ints: usize = 0;
    let mut num_lists: u64 = 0;
    let mut list_size: u64 = 0;
    let mut prev_x: u64 = 0;
    let mut x: u64 = 0;

    let file = File::open(input_lists_filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        if list_size == 0 {
            list_size = line.unwrap().parse().unwrap();
            println!("list-{} size = {}", num_lists, list_size);
            builder.append_bits(list_size, 32);
            num_lists += 1;
            prev_x = 0;
            x = 0;
        } else {
            x = line.unwrap().parse().unwrap();
            assert!(x >= prev_x);
            C::write(&mut builder, x - prev_x);
            num_ints += 1;
            list_size -= 1;
            prev_x = x;
        }
    });

    builder.set_bits(0, num_lists, 32);

    println!("compressed {} lists", num_lists);
    println!("({} integers)", num_ints);
    println!("written {} bits", builder.num_bits());
    println!("({} bits/int)", builder.num_bits() as f64 / num_ints as f64);

    let mut bits = BitVector::new();
    builder.build(&mut bits);

    let file = File::create(output_filename).expect("Failed to open output file");
    bits.save(file);
}

#[cfg(test)]
mod tests {
    use super::super::integer_codes::{DeltaCode, GammaCode, RiceCode, VByteCode};
    use super::*;

    #[test]
    fn compress_gamma() {
        compress::<GammaCode, _>("data/lists.txt", "data/out_gamma.bin");
    }

    #[test]
    fn compress_delta() {
        compress::<DeltaCode, _>("data/lists.txt", "data/out_delta.bin");
    }

    #[test]
    fn compress_vbyte() {
        compress::<VByteCode, _>("data/lists.txt", "data/out_vbyte.bin");
    }

    #[test]
    fn compress_rice_k1() {
        compress::<RiceCode<1>, _>("data/lists.txt", "data/out_rice_k1.bin");
    }

    #[test]
    fn compress_rice_k2() {
        compress::<RiceCode<2>, _>("data/lists.txt", "data/out_rice_k2.bin");
    }
}
