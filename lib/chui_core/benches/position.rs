//! Benchmark *Positions.

#![allow(missing_docs)]

use std::time::Duration;

use rand::distributions::{Distribution, Uniform};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
// use criterion::black_box;

use chui_core::prelude::*;

/// Generate a vector of `Coordinate` pairs.
fn gen_coords(num_coords: u64) -> Vec<(Coord, Coord)> {
    let mut rng = rand::thread_rng();
    let coord = Uniform::from(0..8);
    let mut coords: Vec<(Coord, Coord)> = Vec::new();

    for _ in 0..num_coords {
        coords.push((
            Coord::try_from((coord.sample(&mut rng), coord.sample(&mut rng))).unwrap(),
            Coord::try_from((coord.sample(&mut rng), coord.sample(&mut rng))).unwrap(),
        ));
    }

    coords
}

/// Benchmark the input Position.
fn piece_operation(position: &mut dyn Position, coords: &(Coord, Coord)) {
    let p1 = position.get_piece(coords.0);
    let p2 = position.put_piece(p1, coords.1);
    position.put_piece(p2, coords.0);
}

/// `ArrayBitPosition` copies.
const fn array_bit_position_copy(position: ArrayBitPosition) -> ChuiResult<ArrayBitPosition> {
    let pos = position;
    Ok(pos)
}

/// `BitPosition` copies.
const fn bit_position_copy(position: BitPosition) -> ChuiResult<BitPosition> {
    let pos = position;
    Ok(pos)
}

/// `BitPosition` copies.
const fn easy_position_copy(position: EasyPosition) -> ChuiResult<EasyPosition> {
    let pos = position;
    Ok(pos)
}

/// `BitPosition` copies.
const fn enum_position_copy(position: EnumPosition) -> ChuiResult<EnumPosition> {
    let pos = position;
    Ok(pos)
}

/// Piece operations benchmark.
fn piece_operations_benchmark(c: &mut Criterion) {
    let mut array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let mut bit_position = BitPosition::new(Variant::StandardChess);
    let mut easy_position = EasyPosition::new(Variant::StandardChess);
    let mut enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let coords_vec = gen_coords(1_000_000);

    let mut array_bit_position_group = c.benchmark_group("(Piece Operations) ArrayBitPosition");
    array_bit_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    array_bit_position_group.measurement_time(duration);
    array_bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut array_bit_position, coord);
                }
            });
        },
    );
    array_bit_position_group.finish();

    let mut bit_position_group = c.benchmark_group("(Piece Operations) BitPosition");
    bit_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    bit_position_group.measurement_time(duration);
    bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut bit_position, coord);
                }
            });
        },
    );
    bit_position_group.finish();

    let mut easy_position_group = c.benchmark_group("(Piece Operations) EasyPosition");
    easy_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    easy_position_group.measurement_time(duration);
    easy_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut easy_position, coord);
                }
            });
        },
    );
    easy_position_group.finish();

    let mut enum_position_group = c.benchmark_group("(Piece Operations) EnumPosition");
    enum_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    enum_position_group.measurement_time(duration);
    enum_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    piece_operation(&mut enum_position, coord);
                }
            });
        },
    );
    enum_position_group.finish();
}

/// Copy positions benchmark.
fn position_copies_benchmark(c: &mut Criterion) {
    let array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let bit_position = BitPosition::new(Variant::StandardChess);
    let easy_position = EasyPosition::new(Variant::StandardChess);
    let enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let num_copies: u64 = 1_000_000;

    let mut array_bit_position_group = c.benchmark_group("(Position Copies) ArrayBitPosition");
    array_bit_position_group.throughput(Throughput::Elements(num_copies));
    array_bit_position_group.measurement_time(duration);
    array_bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(num_copies),
        &num_copies,
        |b, _| {
            b.iter(|| array_bit_position_copy(array_bit_position));
        },
    );
    array_bit_position_group.finish();

    let mut bit_position_group = c.benchmark_group("(Position Copies) BitPosition");
    bit_position_group.throughput(Throughput::Elements(num_copies));
    bit_position_group.measurement_time(duration);
    bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(num_copies),
        &num_copies,
        |b, _| {
            b.iter(|| bit_position_copy(bit_position));
        },
    );
    bit_position_group.finish();

    let mut easy_position_group = c.benchmark_group("(Position Copies) EasyPosition");
    easy_position_group.throughput(Throughput::Elements(num_copies));
    easy_position_group.measurement_time(duration);
    easy_position_group.bench_with_input(
        BenchmarkId::from_parameter(num_copies),
        &num_copies,
        |b, _| {
            b.iter(|| easy_position_copy(easy_position));
        },
    );
    easy_position_group.finish();

    let mut enum_position_group = c.benchmark_group("(Position Copies) EnumPosition");
    enum_position_group.throughput(Throughput::Elements(num_copies));
    enum_position_group.measurement_time(duration);
    enum_position_group.bench_with_input(
        BenchmarkId::from_parameter(num_copies),
        &num_copies,
        |b, _| {
            b.iter(|| enum_position_copy(enum_position));
        },
    );
    enum_position_group.finish();
}

/// Piece operations benchmark.
fn position_tree_benchmark(c: &mut Criterion) {
    let array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let bit_position = BitPosition::new(Variant::StandardChess);
    let easy_position = EasyPosition::new(Variant::StandardChess);
    let enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let coords_vec = gen_coords(1_000_000);

    let mut array_bit_position_group = c.benchmark_group("(Copy then Operate) ArrayBitPosition");
    array_bit_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    array_bit_position_group.measurement_time(duration);
    array_bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    let mut pos = array_bit_position_copy(array_bit_position).unwrap();
                    piece_operation(&mut pos, coord);
                }
            });
        },
    );
    array_bit_position_group.finish();

    let mut bit_position_group = c.benchmark_group("(Copy then Operate) BitPosition");
    bit_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    bit_position_group.measurement_time(duration);
    bit_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    let mut pos = bit_position_copy(bit_position).unwrap();
                    piece_operation(&mut pos, coord);
                }
            });
        },
    );
    bit_position_group.finish();

    let mut easy_position_group = c.benchmark_group("(Copy then Operate) EasyPosition");
    easy_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    easy_position_group.measurement_time(duration);
    easy_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
        &coords_vec,
        |b, coords| {
            b.iter(|| {
                for coord in coords.iter() {
                    let mut pos = easy_position_copy(easy_position).unwrap();
                    piece_operation(&mut pos, coord);
                }
            });
        },
    );
    easy_position_group.finish();

    let mut enum_position_group = c.benchmark_group("(Copy then Operate) EnumPosition");
    enum_position_group.throughput(Throughput::Elements(coords_vec.len() as u64));
    enum_position_group.measurement_time(duration);
    enum_position_group.bench_with_input(
        BenchmarkId::from_parameter(coords_vec.len()),
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
    enum_position_group.finish();
}

criterion_group!(piece_operations, piece_operations_benchmark);
criterion_group!(position_copies, position_copies_benchmark);
criterion_group!(position_tree, position_tree_benchmark);

// criterion_main!(piece_operations);
// criterion_main!(position_copies);
// criterion_main!(position_tree);

criterion_main!(piece_operations, position_copies, position_tree);
