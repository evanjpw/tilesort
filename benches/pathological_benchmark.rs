use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::prelude::*;
use rand::seq::SliceRandom;
use tilesort::{tilesort, tilesort_by_key};

// TODO: This benchmark uses the accidentally pathological case from the old realistic_workload
// where shuffling independently-generated tiles creates massive overlaps (5.9% of data).
// We should add a controlled pathological case where we deliberately create specific
// overlap patterns to test worst-case behavior.

/// Generate data with sorted tiles - this accidentally creates pathological overlaps
/// when tiles are shuffled because each tile gets values from a distinct range,
/// but those ranges end up interleaved when shuffled.
fn generate_pathological_overlapping_tiles(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
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
    // This creates the pathological case: tiles from different ranges get interleaved
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

/// Structured data for key function benchmarks
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct LogEntry {
    timestamp: u64,
    severity: u8,
    message: String,
}

/// Generate pathological overlapping log entries (sorted by timestamp within tiles, but tiles overlap)
fn generate_overlapping_logs(total_size: usize, tile_sizes: &[usize]) -> Vec<LogEntry> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);

        // Generate a sorted tile by timestamp - random starting values create overlaps
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

fn bench_pathological_overlap(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathological_overlap");

    // This is the accidentally pathological case: ~10K row tiles, ~1M total rows
    // The shuffling creates ~59K duplicate values (5.9% of data)
    let size = 1_000_000;
    let tile_sizes = vec![8000, 10000, 12000, 9000, 11000];

    group.bench_function("tilesort_1M", |b| {
        b.iter_batched(
            || generate_pathological_overlapping_tiles(size, &tile_sizes),
            |mut data| tilesort(black_box(&mut data)),
            BatchSize::LargeInput,
        )
    });

    group.bench_function("std_sort_1M", |b| {
        b.iter_batched(
            || generate_pathological_overlapping_tiles(size, &tile_sizes),
            |mut data| data.sort(),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

fn bench_pathological_key_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathological_key_function");

    for size in [1_000, 10_000, 100_000].iter() {
        let tile_sizes = vec![100, 1000, 5000];

        group.bench_with_input(BenchmarkId::new("tilesort", size), size, |b, &size| {
            b.iter_batched(
                || generate_overlapping_logs(size, &tile_sizes),
                |mut data| tilesort_by_key(black_box(&mut data), |log| log.timestamp),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &size| {
            b.iter_batched(
                || generate_overlapping_logs(size, &tile_sizes),
                |mut data| data.sort_by_key(|log| log.timestamp),
                BatchSize::LargeInput,
            )
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_pathological_overlap,
    bench_pathological_key_function,
);

criterion_main!(benches);
