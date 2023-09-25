use super::bit_vector_iterator::BitVectorIterator;
use crate::introduction::bit_vector::BitVectorBuilder;
use crate::introduction::util::msb;

pub fn write_unary(builder: &mut BitVectorBuilder, x: usize) {
    assert!(x < 64);
    let u = 1u64 << x;
    builder.append_bits(u, x + 1);
}

pub fn read_unary(it: &mut BitVectorIterator) -> usize {
    it.skip_zeros()
}

/// write the integer x <= r using b=ceil(log2(r+1)) bits
pub fn write_binary(builder: &mut BitVectorBuilder, x: u64, r: u64) {
    assert!(r > 0);
    assert!(x <= r);
    let b = msb(r) + 1;
    builder.append_bits(x, b);
}

/// read b=ceil(log2(r+1)) bits and interprets them as the integer x
pub fn read_binary(it: &mut BitVectorIterator, r: u64) -> u64 {
    assert!(r > 0);
    let b = msb(r) + 1;
    let x = it.take(b);
    assert!(x <= r);
    x
}

pub fn write_gamma(builder: &mut BitVectorBuilder, x: u64) {
    let xx = x + 1;
    let b = msb(xx);
    write_unary(builder, b);
    let mask = (1u64 << b).wrapping_sub(1);
    builder.append_bits(xx & mask, b);
}

pub fn read_gamma(it: &mut BitVectorIterator) -> u64 {
    let b = read_unary(it);
    (it.take(b) | (1 << b)).wrapping_sub(1)
}

pub fn write_delta(builder: &mut BitVectorBuilder, x: u64) {
    let xx = x + 1;
    let b = msb(xx);
    write_gamma(builder, b as u64);
    let mask = (1u64 << b).wrapping_sub(1);
    builder.append_bits(xx & mask, b);
}

pub fn read_delta(it: &mut BitVectorIterator) -> u64 {
    let b = read_gamma(it) as usize;
    (it.take(b) | (1 << b)).wrapping_sub(1)
}

pub fn write_rice(builder: &mut BitVectorBuilder, x: u64, k: usize) {
    assert!(k > 0);
    let q = x >> k;
    let r = x - (q << k);
    write_gamma(builder, q);
    builder.append_bits(r, k);
}

pub fn read_rice(it: &mut BitVectorIterator, k: usize) -> u64 {
    assert!(k > 0);
    let q = read_gamma(it);
    let r = it.take(k);
    r + (q << k)
}

pub fn write_vbyte(builder: &mut BitVectorBuilder, x: u64) {
    if x < 128 {
        builder.append_bits(x, 8);
        return;
    }
    let data_bits = x & 127;
    builder.append_bits(data_bits | 128, 8);
    write_vbyte(builder, x >> 7);
}

pub fn read_vbyte(it: &mut BitVectorIterator) -> u64 {
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
