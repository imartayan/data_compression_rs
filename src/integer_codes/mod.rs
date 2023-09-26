pub mod bit_vector_iterator;
pub mod check;
pub mod compress;
pub mod decompress;

use crate::introduction::bit_vector::BitVectorBuilder;
use crate::introduction::util::msb;
use bit_vector_iterator::BitVectorIterator;
use num_traits::int::PrimInt;

pub trait IntegerCode<T: PrimInt> {
    fn write(builder: &mut BitVectorBuilder, x: T);
    fn read(it: &mut BitVectorIterator) -> T;
}

pub struct UnaryCode;

impl IntegerCode<usize> for UnaryCode {
    fn write(builder: &mut BitVectorBuilder, x: usize) {
        assert!(x < 64);
        let u = 1u64 << x;
        builder.append_bits(u, x + 1);
    }

    fn read(it: &mut BitVectorIterator) -> usize {
        it.skip_zeros()
    }
}

pub struct BinaryCode<const R: u64>;

impl<const R: u64> IntegerCode<u64> for BinaryCode<R> {
    /// write the integer x <= r using b=ceil(log2(r+1)) bits
    fn write(builder: &mut BitVectorBuilder, x: u64) {
        assert!(R > 0);
        assert!(x <= R);
        let b = msb(R) + 1;
        builder.append_bits(x, b);
    }

    /// read b=ceil(log2(r+1)) bits and interprets them as the integer x
    fn read(it: &mut BitVectorIterator) -> u64 {
        assert!(R > 0);
        let b = msb(R) + 1;
        let x = it.take(b);
        assert!(x <= R);
        x
    }
}

pub struct GammaCode;

impl IntegerCode<u64> for GammaCode {
    fn write(builder: &mut BitVectorBuilder, x: u64) {
        let xx = x + 1;
        let b = msb(xx);
        UnaryCode::write(builder, b);
        let mask = (1u64 << b).wrapping_sub(1);
        builder.append_bits(xx & mask, b);
    }

    fn read(it: &mut BitVectorIterator) -> u64 {
        let b = UnaryCode::read(it);
        (it.take(b) | (1 << b)).wrapping_sub(1)
    }
}

pub struct DeltaCode;

impl IntegerCode<u64> for DeltaCode {
    fn write(builder: &mut BitVectorBuilder, x: u64) {
        let xx = x + 1;
        let b = msb(xx);
        GammaCode::write(builder, b as u64);
        let mask = (1u64 << b).wrapping_sub(1);
        builder.append_bits(xx & mask, b);
    }

    fn read(it: &mut BitVectorIterator) -> u64 {
        let b = GammaCode::read(it) as usize;
        (it.take(b) | (1 << b)).wrapping_sub(1)
    }
}

pub struct RiceCode<const K: usize>;

impl<const K: usize> IntegerCode<u64> for RiceCode<K> {
    fn write(builder: &mut BitVectorBuilder, x: u64) {
        assert!(K > 0);
        let q = x >> K;
        let r = x - (q << K);
        GammaCode::write(builder, q);
        builder.append_bits(r, K);
    }

    fn read(it: &mut BitVectorIterator) -> u64 {
        assert!(K > 0);
        let q = GammaCode::read(it);
        let r = it.take(K);
        r + (q << K)
    }
}

pub struct VByteCode;

impl IntegerCode<u64> for VByteCode {
    fn write(builder: &mut BitVectorBuilder, x: u64) {
        if x < 128 {
            builder.append_bits(x, 8);
            return;
        }
        let data_bits = x & 127;
        builder.append_bits(data_bits | 128, 8);
        Self::write(builder, x >> 7);
    }

    fn read(it: &mut BitVectorIterator) -> u64 {
        let mut val = 0;
        let mut shift = 0;
        loop {
            let byte = it.take_one_byte();
            val += (byte & 127) << shift;
            if byte < 128 {
                break;
            }
            shift += 7;
        }
        val
    }
}
