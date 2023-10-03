//! Benchmark `ArrayBitPosition`.

use criterion::{BenchmarkId, Criterion, Throughput};

use chui_core::prelude::*;

/// Piece operations benchmark.
fn piece_operations(c: &mut Criterion, n: u64) {
    let mut array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let coords_vec = gen_coords(n);
    let mut group = c.benchmark_group("ArrayBitPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Piece Operations"),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut array_bit_position, coord);
                }
            });
        },
    );
    group.finish();
}

/// Copy positions benchmark.
fn position_copies(c: &mut Criterion, n: u64) {
    let array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let mut group = c.benchmark_group("ArrayBitPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Position Copies"),
        &n,
        |b, _| {
            b.iter(|| array_bit_position_copy(&array_bit_position));
        },
    );
    group.finish();
}

/// Piece operations benchmark.
fn position_tree(c: &mut Criterion, n: u64) {
    let array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let coords_vec = gen_coords(n);
    let mut group = c.benchmark_group("ArrayBitPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Copy then Operate"),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(
                        &mut array_bit_position_copy(&array_bit_position).unwrap(),
                        coord,
                    );
                }
            });
        },
    );
    group.finish();
}

/// `ArrayBitPosition` benchmarks.
pub fn array_bit_position_benchmarks(c: &mut Criterion) {
    let n = 1_u64;
    piece_operations(c, n);
    position_copies(c, n);
    position_tree(c, n);
}