//! Benchmark `BitSetPosition`.

use criterion::{BenchmarkId, Criterion, Throughput};

use chui_core::prelude::*;

/// Piece operations benchmark.
fn piece_operations(c: &mut Criterion, n: u64) {
    let mut bit_position = BitSetPosition::new(Variant::StandardChess);
    let coords_vec = gen_coords(n);
    let mut group = c.benchmark_group("BitSetPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Piece Operations"),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut bit_position, coord);
                }
            });
        },
    );
    group.finish();
}

/// Copy positions benchmark.
fn position_copies(c: &mut Criterion, n: u64) {
    let bit_position = BitSetPosition::new(Variant::StandardChess);
    let mut group = c.benchmark_group("BitSetPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Position Copies"),
        &n,
        |b, _| {
            b.iter(|| bitset_position_copy(&bit_position));
        },
    );
    group.finish();
}

/// Piece operations benchmark.
fn position_tree(c: &mut Criterion, n: u64) {
    let bit_position = BitSetPosition::new(Variant::StandardChess);
    let coords_vec = gen_coords(n);
    let mut group = c.benchmark_group("BitSetPosition");
    group.throughput(Throughput::Elements(n));
    group.bench_with_input(
        BenchmarkId::from_parameter("Copy then Operate"),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut bitset_position_copy(&bit_position).unwrap(), coord);
                }
            });
        },
    );
    group.finish();
}

/// `BitSetPosition` benchmarks.
pub fn bitset_position_benchmarks(c: &mut Criterion) {
    let n = 1_u64;
    piece_operations(c, n);
    position_copies(c, n);
    position_tree(c, n);
}
