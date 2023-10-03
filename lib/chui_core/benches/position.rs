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
fn bench_position(position: &mut dyn Position, coords: &[(Coord, Coord)]) -> ChuiResult<()> {
    for (c1, c2) in coords.iter() {
        let p1 = position.get_piece(*c1);
        let p2 = position.put_piece(p1, *c2);
        position.put_piece(p2, *c1);
    }
    Ok(())
}

/// Criterion benchmark function.
fn criterion_benchmark(c: &mut Criterion) {
    let mut array_bit_position = ArrayBitPosition::new(Variant::StandardChess);
    let mut bit_position = BitPosition::new(Variant::StandardChess);
    let mut easy_position = EasyPosition::new(Variant::StandardChess);
    let mut enum_position = EnumPosition::new(Variant::StandardChess);
    let duration = Duration::new(15, 0);
    let coords_vec = [
        gen_coords(10),
        gen_coords(100),
        gen_coords(1_000),
        gen_coords(10_000),
        gen_coords(100_000),
        gen_coords(1_000_000),
    ];

    let mut array_bit_position_group = c.benchmark_group("ArrayBitPosition");
    for coords in coords_vec.iter() {
        array_bit_position_group.throughput(Throughput::Elements(coords.len() as u64));
        array_bit_position_group.measurement_time(duration);
        array_bit_position_group.bench_with_input(
            BenchmarkId::from_parameter(coords.len()),
            coords,
            |b, vec| {
                b.iter(|| bench_position(&mut array_bit_position, vec));
            },
        );
    }
    array_bit_position_group.finish();

    let mut bit_position_group = c.benchmark_group("BitPosition");
    for coords in coords_vec.iter() {
        bit_position_group.throughput(Throughput::Elements(coords.len() as u64));
        bit_position_group.measurement_time(duration);
        bit_position_group.bench_with_input(
            BenchmarkId::from_parameter(coords.len()),
            coords,
            |b, vec| {
                b.iter(|| bench_position(&mut bit_position, vec));
            },
        );
    }
    bit_position_group.finish();

    let mut easy_position_group = c.benchmark_group("EasyPosition");
    for coords in coords_vec.iter() {
        easy_position_group.throughput(Throughput::Elements(coords.len() as u64));
        easy_position_group.measurement_time(duration);
        easy_position_group.bench_with_input(
            BenchmarkId::from_parameter(coords.len()),
            coords,
            |b, vec| {
                b.iter(|| bench_position(&mut easy_position, vec));
            },
        );
    }
    easy_position_group.finish();

    let mut enum_position_group = c.benchmark_group("EnumPosition");
    for coords in coords_vec.iter() {
        enum_position_group.throughput(Throughput::Elements(coords.len() as u64));
        enum_position_group.measurement_time(duration);
        enum_position_group.bench_with_input(
            BenchmarkId::from_parameter(coords.len()),
            coords,
            |b, vec| {
                b.iter(|| bench_position(&mut enum_position, vec));
            },
        );
    }
    enum_position_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
