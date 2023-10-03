//! Benchmark those Positions which implement the `Position` trait.

#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod array_bit_position;
pub use array_bit_position::array_bit_position_benchmarks;

mod bit_position;
pub use bit_position::bit_position_benchmarks;

mod bitset_position;
pub use bitset_position::bitset_position_benchmarks;

mod easy_2d_position;
pub use easy_2d_position::easy_2d_position_benchmarks;

mod easy_1d_position;
pub use easy_1d_position::easy_1d_position_benchmarks;

mod enum_position;
pub use enum_position::enum_position_benchmarks;

criterion_group!(
    // Group name.
    position_benchmarks,
    // Group of benchmarks.
    array_bit_position_benchmarks,
    bit_position_benchmarks,
    bitset_position_benchmarks,
    enum_position_benchmarks,
    easy_1d_position_benchmarks,
    easy_2d_position_benchmarks,
);

criterion_main!(position_benchmarks);
