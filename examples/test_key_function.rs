use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::time::Instant;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct LogEntry {
    timestamp: u64,
    severity: u8,
    message: String,
}

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

fn main() {
    let size = 100_000;
    let tile_sizes = vec![100, 1000, 5000];

    println!("Generating key_function test data (100K log entries)...");
    let data_original = generate_tiled_logs(size, &tile_sizes);

    println!("Data size: {} entries", data_original.len());

    // Detect tiles by finding runs where timestamp decreases
    let mut tile_boundaries = vec![0];
    for i in 0..data_original.len() - 1 {
        if data_original[i].timestamp > data_original[i + 1].timestamp {
            tile_boundaries.push(i + 1);
        }
    }
    tile_boundaries.push(data_original.len());

    println!("\nDetected {} tiles", tile_boundaries.len() - 1);

    // Analyze tile ranges
    let mut tile_ranges: Vec<(usize, u64, u64)> = Vec::new();
    for i in 0..tile_boundaries.len() - 1 {
        let start_idx = tile_boundaries[i];
        let end_idx = tile_boundaries[i + 1];
        let min_ts = data_original[start_idx].timestamp;
        let max_ts = data_original[end_idx - 1].timestamp;
        tile_ranges.push((i, min_ts, max_ts));
        if i < 10 {
            println!(
                "  Tile {}: size {}, timestamps {}-{}",
                i,
                end_idx - start_idx,
                min_ts,
                max_ts
            );
        }
    }

    // Count actual overlapping timestamps
    let mut timestamp_counts: std::collections::HashMap<u64, Vec<usize>> =
        std::collections::HashMap::new();

    for tile_idx in 0..tile_boundaries.len() - 1 {
        let start_idx = tile_boundaries[tile_idx];
        let end_idx = tile_boundaries[tile_idx + 1];
        for i in start_idx..end_idx {
            timestamp_counts
                .entry(data_original[i].timestamp)
                .or_insert_with(Vec::new)
                .push(tile_idx);
        }
    }

    let overlapping_timestamps: Vec<_> = timestamp_counts
        .iter()
        .filter(|(_, tiles)| tiles.len() > 1)
        .collect();

    println!(
        "\nActual overlapping timestamps: {}",
        overlapping_timestamps.len()
    );
    if !overlapping_timestamps.is_empty() {
        println!("First 5 overlapping timestamps:");
        for (ts, tiles) in overlapping_timestamps.iter().take(5) {
            println!("  Timestamp {} appears in tiles: {:?}", ts, tiles);
        }
    }

    println!("\nSorting with tilesort...");
    let start = Instant::now();
    let mut data = data_original.clone();
    tilesort::tilesort_by_key(&mut data, |log| log.timestamp);
    let elapsed = start.elapsed();

    println!("Tilesort took: {:?}", elapsed);

    // Verify sorted
    let mut errors = 0;
    for i in 0..data.len() - 1 {
        if data[i].timestamp > data[i + 1].timestamp {
            if errors < 10 {
                println!(
                    "ERROR at index {}: {} > {}",
                    i,
                    data[i].timestamp,
                    data[i + 1].timestamp
                );
            }
            errors += 1;
        }
    }

    if errors == 0 {
        println!("✓ Data is correctly sorted!");
    } else {
        println!("✗ FAILED: {} sorting errors found", errors);
    }
}
