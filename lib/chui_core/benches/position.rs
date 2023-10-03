//! Benchmark those Positions which implement the `Position` trait.

#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod array_bit_position;
pub use array_bit_position::array_bit_position_benchmarks;

mod bit_position;
pub use bit_position::bit_position_benchmarks;

mod easy_position;
pub use easy_position::easy_position_benchmarks;

mod enum_position;
pub use enum_position::enum_position_benchmarks;

criterion_group!(
    position_benchmarks,
    array_bit_position_benchmarks,
    bit_position_benchmarks,
    easy_position_benchmarks,
    enum_position_benchmarks,
);

criterion_main!(position_benchmarks);
