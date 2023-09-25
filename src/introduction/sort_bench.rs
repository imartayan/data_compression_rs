use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LargeRecord {
    weight: u64,
    height: u64,
    day: u64,
    month: u64,
    year: u64,
}

impl LargeRecord {
    pub fn new(weight: u64, height: u64, day: u64, month: u64, year: u64) -> Self {
        Self {
            weight,
            height,
            day,
            month,
            year,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallRecord {
    weight: u8,
    height: u8,
    day: u8,
    month: u8,
    year: u16,
}

impl SmallRecord {
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
fn sort_bench() {
    let vector_size = 10_000_000;
    let mut rng = SmallRng::seed_from_u64(13);
    let mut large_records: Vec<LargeRecord> = Vec::new();
    let mut small_records: Vec<SmallRecord> = Vec::new();
    large_records.reserve(vector_size);
    small_records.reserve(vector_size);
    for _ in 0..vector_size {
        let weight = rng.gen::<u64>() % 256;
        let height = rng.gen::<u64>() % 256;
        let day = rng.gen::<u64>() % 32;
        let month = rng.gen::<u64>() % 16;
        let year = rng.gen::<u64>() % 4096;
        large_records.push(LargeRecord::new(weight, height, day, month, year));
        small_records.push(SmallRecord::new(
            weight as u8,
            height as u8,
            day as u8,
            month as u8,
            year as u16,
        ));
    }
    let now = Instant::now();
    large_records.sort_unstable();
    let elapsed = now.elapsed().as_millis();
    println!("sorting vec took: {} ms", elapsed);
    let now = Instant::now();
    small_records.sort_unstable();
    let elapsed = now.elapsed().as_millis();
    println!("sorting vec took: {} ms", elapsed);
}
