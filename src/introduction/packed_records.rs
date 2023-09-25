use super::bit_vector::{BitVector, BitVectorBuilder};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Record {
    weight: u8,
    height: u8,
    day: u8,
    month: u8,
    year: u16,
}

impl Record {
    pub fn new(weight: u8, height: u8, day: u8, month: u8, year: u16) -> Self {
        Self {
            weight,
            height,
            day,
            month,
            year,
        }
    }
}

#[test]
fn packed_records() {
    let n = 10000;
    let mut records: Vec<Record> = Vec::new();
    records.reserve(n);

    let mut rng = SmallRng::seed_from_u64(3);
    for _ in 0..n {
        let r = Record::new(
            rng.gen(),
            rng.gen(),
            rng.gen::<u8>() % 32,
            rng.gen::<u8>() % 16,
            rng.gen::<u16>() % 4096,
        );
        records.push(r);
    }

    let mut builder = BitVectorBuilder::new();
    builder.reserve(37 * n);

    for r in &records {
        builder.append_bits(r.weight as u64, 8);
        builder.append_bits(r.height as u64, 8);
        builder.append_bits(r.day as u64, 5);
        builder.append_bits(r.month as u64, 4);
        builder.append_bits(r.year as u64, 12);
    }

    let mut packed_records = BitVector::new();
    builder.build(&mut packed_records);

    println!("packed_records num_bits = {}", packed_records.num_bits());
    println!(
        "{} bits/record",
        packed_records.num_bits() as f64 / n as f64
    );
    println!(
        "packed_records num_64bit_words = {}",
        packed_records.num_64bit_words()
    );
    println!(
        "{} bits/record",
        packed_records.num_64bit_words() as f64 * 64.0 / n as f64
    );

    let mut pos = 0;
    for r in &records {
        let weight = packed_records.get_bits(pos, 8) as u8;
        assert_eq!(weight, r.weight);
        pos += 8;

        let height = packed_records.get_bits(pos, 8) as u8;
        assert_eq!(height, r.height);
        pos += 8;

        let day = packed_records.get_bits(pos, 5) as u8;
        assert_eq!(day, r.day);
        pos += 5;

        let month = packed_records.get_bits(pos, 4) as u8;
        assert_eq!(month, r.month);
        pos += 4;

        let year = packed_records.get_bits(pos, 12) as u16;
        assert_eq!(year, r.year);
        pos += 12;
    }
}
