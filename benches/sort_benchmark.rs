use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::prelude::*;
use tilesort::{tilesort, tilesort_by_key};

/// Generate data with sorted tiles - non-overlapping ranges (primary use case)
fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    // Assign non-overlapping ranges: each tile gets a distinct range
    // Spacing is large enough to ensure no overlap
    let spacing = 10_000;

    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);
        let start: i32 = (tile_idx as i32) * spacing;
        let mut tile: Vec<i32> = (0..tile_size).map(|i| start + i as i32).collect();

        result.append(&mut tile);
        remaining -= tile_size;
        tile_idx += 1;
    }

    // Shuffle the tiles (keeping each tile internally sorted)
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

#[allow(dead_code)]
/// Generate realistic tiled data: sequential values split into tiles, then shuffled
fn generate_minimally_overlapping_tiles(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);

    // Generate sequential data [0, 1, 2, ..., total_size-1]
    let data: Vec<i32> = (0..total_size as i32).collect();

    // Split into tiles
    let mut tiles = Vec::new();
    let mut pos = 0;
    for &size in tile_sizes.iter().cycle() {
        if pos >= data.len() {
            break;
        }
        let end = (pos + size).min(data.len());
        tiles.push(data[pos..end].to_vec());
        pos = end;
    }

    // Shuffle tiles (not elements within tiles)
    tiles.shuffle(&mut rng);
    tiles.into_iter().flatten().collect()
}

/// Generate data with substantial overlap - worst case for tilesort
#[allow(dead_code)]
fn generate_overlapping_tiles(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    // Random starting values in small range create substantial overlaps
    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);
        let start: i32 = rng.random_range(0..100_000);
        let mut tile: Vec<i32> = (0..tile_size).map(|i| start + i as i32).collect();

        result.append(&mut tile);
        remaining -= tile_size;
        tile_idx += 1;
    }

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

/// Generate realistic minimally overlapping log entries (like realistic_workload but with LogEntry structs)
/// Sequential timestamps split into tiles, then shuffled
fn generate_minimally_overlapping_logs(total_size: usize, tile_sizes: &[usize]) -> Vec<LogEntry> {
    let mut rng = StdRng::seed_from_u64(42);

    // Generate sequential timestamps
    let mut logs: Vec<LogEntry> = (0..total_size)
        .map(|i| LogEntry {
            timestamp: i as u64 * 1000,
            severity: rng.random_range(0..5),
            message: format!("Log message {}", i),
        })
        .collect();

    // Split into tiles
    let mut tiles = Vec::new();
    let mut pos = 0;
    for &size in tile_sizes.iter().cycle() {
        if pos >= logs.len() {
            break;
        }
        let end = (pos + size).min(logs.len());
        tiles.push(logs[pos..end].to_vec());
        pos = end;
    }

    // Shuffle tiles (not elements within tiles)
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
                || generate_minimally_overlapping_logs(size, &tile_sizes),
                |mut data| tilesort_by_key(black_box(&mut data), |log| log.timestamp),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_minimally_overlapping_logs(size, &tile_sizes),
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
    // Each tile represents logs from a different source/time period with no overlaps
    let size = 1_000_000;
    let tile_sizes = vec![8000, 10000, 12000, 9000, 11000]; // Varied around 10K

    group.bench_function("tilesort_1M", |b| {
        b.iter_batched(
            || generate_minimally_overlapping_tiles(size, &tile_sizes),
            |mut data| tilesort(black_box(&mut data)),
            BatchSize::LargeInput,
        )
    });

    group.bench_function("std_sort_1M", |b| {
        b.iter_batched(
            || generate_minimally_overlapping_tiles(size, &tile_sizes),
            |mut data| data.sort(),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

fn bench_many_uniform_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("many_uniform_tiles");

    // Testing with 1000 uniform tiles (~1000 elements each = 1M total)
    let size = 1_000_000;
    let tile_sizes = vec![1000]; // 1000 tiles of 1000 elements each

    group.bench_function("tilesort_1000tiles", |b| {
        b.iter_batched(
            || generate_tiled_data(size, &tile_sizes),
            |mut data| tilesort(black_box(&mut data)),
            BatchSize::LargeInput,
        )
    });

    group.bench_function("std_sort_1000tiles", |b| {
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
    bench_many_uniform_tiles,
);

criterion_main!(benches);
