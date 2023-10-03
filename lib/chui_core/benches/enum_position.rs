//! Benchmark `EnumPosition`.

use std::time::Duration;

use criterion::{BenchmarkId, Criterion, Throughput};

use chui_core::prelude::*;

/// Piece operations benchmark.
fn piece_operations(c: &mut Criterion) {
    let mut enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let coords_vec = gen_coords(1_000_000);
    let formatted_len = num_sep(coords_vec.len().to_string().as_str(), Some('_'))
        .map_or(coords_vec.len().to_string(), |v| v);

    let mut group = c.benchmark_group("EnumPosition");
    group.throughput(Throughput::Elements(coords_vec.len() as u64));
    group.measurement_time(duration);
    group.bench_with_input(
        BenchmarkId::from_parameter(format!("Piece Operations = ({})", formatted_len)),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut enum_position, coord);
                }
            });
        },
    );
    group.finish();
}

/// Copy positions benchmark.
fn position_copies(c: &mut Criterion) {
    let enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let num_copies: u64 = 1_000_000;
    let formatted_len =
        num_sep(num_copies.to_string().as_str(), Some('_')).map_or(num_copies.to_string(), |v| v);

    let mut group = c.benchmark_group("EnumPosition");
    group.throughput(Throughput::Elements(num_copies));
    group.measurement_time(duration);
    group.bench_with_input(
        BenchmarkId::from_parameter(format!("Position Copies = ({})", formatted_len)),
        &num_copies,
        |b, _| {
            b.iter(|| enum_position_copy(enum_position));
        },
    );
    group.finish();
}

/// Piece operations benchmark.
fn position_tree(c: &mut Criterion) {
    let enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let coords_vec = gen_coords(1_000_000);
    let formatted_len = num_sep(coords_vec.len().to_string().as_str(), Some('_'))
        .map_or(coords_vec.len().to_string(), |v| v);

    let mut group = c.benchmark_group("EnumPosition");
    group.throughput(Throughput::Elements(coords_vec.len() as u64));
    group.measurement_time(duration);
    group.bench_with_input(
        BenchmarkId::from_parameter(format!("Copy then Operate ({})", formatted_len)),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    let mut pos = enum_position_copy(enum_position).unwrap();
                    piece_operation(&mut pos, coord);
                }
            });
        },
    );
    group.finish();
}

/// `EnumPosition` benchmarks.
pub fn enum_position_benchmarks(c: &mut Criterion) {
    piece_operations(c);
    position_copies(c);
    position_tree(c);
}
