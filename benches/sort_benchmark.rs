use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::prelude::*;
use tilesort::{tilesort, tilesort_by_key};

/// Generate data with sorted tiles of varying sizes
fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42); // Deterministic for reproducibility
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);

        // Generate a sorted tile with random starting value
        let start: i32 = rng.random_range(0..1_000_000);
        let mut tile: Vec<i32> = (0..tile_size).map(|i| start + i as i32).collect();

        result.append(&mut tile);
        remaining -= tile_size;
        tile_idx += 1;
    }

    // Shuffle the tiles (but keep each tile internally sorted)
    let mut tiles = Vec::new();
    let mut pos = 0;
    for &size in tile_sizes.iter().cycle() {
        if pos >= result.len() {
            break;
        }
        let end = (pos + size).min(result.len());
        tiles.push(result[pos..end].to_vec());
        pos = end;
    }

    tiles.shuffle(&mut rng);
    tiles.into_iter().flatten().collect()
}

/// Generate completely random data (worst case for tilesort)
fn generate_random_data(size: usize) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..size).map(|_| rng.random()).collect()
}

/// Structured data for key function benchmarks
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct LogEntry {
    timestamp: u64,
    severity: u8,
    message: String,
}

/// Generate tiled log entries (sorted by timestamp within tiles)
fn generate_tiled_logs(total_size: usize, tile_sizes: &[usize]) -> Vec<LogEntry> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);

        // Generate a sorted tile by timestamp
        let start_ts: u64 = rng.random_range(0..1_000_000_000);
        for i in 0..tile_size {
            result.push(LogEntry {
                timestamp: start_ts + (i as u64 * 1000),
                severity: rng.random_range(0..5),
                message: format!("Log message {}", i),
            });
        }

        remaining -= tile_size;
        tile_idx += 1;
    }

    // Shuffle tiles while keeping each tile sorted by timestamp
    let mut tiles = Vec::new();
    let mut pos = 0;
    for &size in tile_sizes.iter().cycle() {
        if pos >= result.len() {
            break;
        }
        let end = (pos + size).min(result.len());
        tiles.push(result[pos..end].to_vec());
        pos = end;
    }

    tiles.shuffle(&mut rng);
    tiles.into_iter().flatten().collect()
}

fn bench_uniform_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("uniform_tiles");

    for size in [1_000, 10_000, 100_000].iter() {
        let tile_size = 1000; // Uniform 1K tiles
        let tile_sizes = vec![tile_size];

        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| tilesort(black_box(&mut data)),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| data.sort(),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

fn bench_varied_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("varied_tiles");

    for size in [1_000, 10_000, 100_000].iter() {
        // Varied tile sizes: small, medium, large
        let tile_sizes = vec![100, 1000, 5000, 10000];

        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| tilesort(black_box(&mut data)),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| data.sort(),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

fn bench_hybrid_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("hybrid_tiles");

    for size in [1_000, 10_000, 100_000].iter() {
        // Radically different sizes: single elements mixed with large blocks
        let tile_sizes = vec![1, 1, 1, 100, 1, 5000, 1, 10000];

        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| tilesort(black_box(&mut data)),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_data(size, &tile_sizes),
                |mut data| data.sort(),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

fn bench_random_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_data");

    for size in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_random_data(size),
                |mut data| tilesort(black_box(&mut data)),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_random_data(size),
                |mut data| data.sort(),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

fn bench_with_key_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_function");

    for size in [1_000, 10_000, 100_000].iter() {
        let tile_sizes = vec![100, 1000, 5000];

        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_logs(size, &tile_sizes),
                |mut data| tilesort_by_key(black_box(&mut data), |log| log.timestamp),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_tiled_logs(size, &tile_sizes),
                |mut data| data.sort_by_key(|log| log.timestamp),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

fn bench_realistic_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_workload");

    // Simulating the user's real data: ~10K row tiles, ~1M total rows
    let size = 1_000_000;
    let tile_sizes = vec![8000, 10000, 12000, 9000, 11000]; // Varied around 10K

    group.bench_function("tilesort_1M", |b| {
        b.iter_batched(
            || generate_tiled_data(size, &tile_sizes),
            |mut data| tilesort(black_box(&mut data)),
            BatchSize::LargeInput,
        )
    });

    group.bench_function("std_sort_1M", |b| {
        b.iter_batched(
            || generate_tiled_data(size, &tile_sizes),
            |mut data| data.sort(),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_uniform_tiles,
    bench_varied_tiles,
    bench_hybrid_tiles,
    bench_random_data,
    bench_with_key_function,
    bench_realistic_workload,
);

criterion_main!(benches);
