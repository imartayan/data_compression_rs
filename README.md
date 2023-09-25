# Crash Course on Data Compression — Rust version

This is a work-in-progress Rust implementation of [Giulio Ermanno Pibiri's course](https://github.com/jermp/data_compression_course/) on data compression.
I do it mainly to have a better understanding of the course, it's not meant to be particularly optimized.

## Running the code

Each of the original examples has been moved to a dedicated test.
In order to execute them all, just run:
```sh
cargo t -- --show-output
```
You can also run a specific one, say `packed_records`, as follows:
```sh
cargo t packed_records -- --show-output
```

## Design choices

I try to stick to the original code whenever possible — some of the design choices I made include:
- using [`smallrng`](https://rust-random.github.io/rand/rand/rngs/struct.SmallRng.html) for random number generation
- using [`serde`](https://serde.rs/) and [`bincode`](https://docs.rs/bincode/) for serialization
- using [`num-traits`](https://docs.rs/num-traits/) to have type-generic `msb` and `lsb` functions
- avoiding architecture-specific instructions (I want the code to work on ARM architectures as well)
