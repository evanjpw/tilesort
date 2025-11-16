use rand::rngs::StdRng;
use rand::SeedableRng;
use std::time::Instant;

fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut _rng = StdRng::seed_from_u64(42);
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

    use rand::seq::SliceRandom;
    let mut rng = StdRng::seed_from_u64(43);
    tiles.shuffle(&mut rng);

    tiles.into_iter().flatten().collect()
}

fn main() {
    let size = 1_000_000;
    let tile_sizes = vec![8000, 10000, 12000, 9000, 11000];

    println!("Generating data for realistic_workload test...");
    let data_original = generate_tiled_data(size, &tile_sizes);

    // Analyze the data to find tiles
    println!("Data size: {} elements", data_original.len());
    println!("First 20 elements: {:?}", &data_original[..20]);

    // Detect tiles by finding runs where data[i] < data[i+1]
    let mut tile_boundaries = vec![0];
    for i in 0..data_original.len() - 1 {
        if data_original[i] > data_original[i + 1] {
            tile_boundaries.push(i + 1);
        }
    }
    tile_boundaries.push(data_original.len());

    println!("\nDetected {} tiles", tile_boundaries.len() - 1);

    // Check for overlapping value ranges
    let mut tile_ranges: Vec<(usize, i32, i32)> = Vec::new();
    for i in 0..tile_boundaries.len() - 1 {
        let start_idx = tile_boundaries[i];
        let end_idx = tile_boundaries[i + 1];
        let min_val = data_original[start_idx];
        let max_val = data_original[end_idx - 1];
        tile_ranges.push((i, min_val, max_val));
        if i < 10 {
            println!(
                "  Tile {}: indices {}-{}, values {}-{}",
                i,
                start_idx,
                end_idx - 1,
                min_val,
                max_val
            );
        }
    }

    // Count actual overlapping rows (how many values appear in multiple tiles)
    let mut value_counts: std::collections::HashMap<i32, Vec<usize>> =
        std::collections::HashMap::new();

    for tile_idx in 0..tile_boundaries.len() - 1 {
        let start_idx = tile_boundaries[tile_idx];
        let end_idx = tile_boundaries[tile_idx + 1];
        for i in start_idx..end_idx {
            value_counts
                .entry(data_original[i])
                .or_insert_with(Vec::new)
                .push(tile_idx);
        }
    }

    let overlapping_values: Vec<_> = value_counts
        .iter()
        .filter(|(_, tiles)| tiles.len() > 1)
        .collect();

    println!(
        "\nActual overlapping values (duplicate rows): {}",
        overlapping_values.len()
    );
    if !overlapping_values.is_empty() {
        println!("First 5 overlapping values:");
        for (val, tiles) in overlapping_values.iter().take(5) {
            println!("  Value {} appears in tiles: {:?}", val, tiles);
        }
    }

    let mut data = data_original.clone();

    println!("\nSorting with tilesort...");
    let start = Instant::now();
    tilesort::tilesort(&mut data);
    let elapsed = start.elapsed();

    println!("Tilesort took: {:?}", elapsed);

    // Verify sorted
    let mut errors = 0;
    for i in 0..data.len() - 1 {
        if data[i] > data[i + 1] {
            if errors < 10 {
                println!("ERROR at index {}: {} > {}", i, data[i], data[i + 1]);
            }
            errors += 1;
        }
    }

    if errors == 0 {
        println!("✓ Data is correctly sorted!");
    } else {
        println!("✗ FAILED: {} sorting errors found", errors);
    }

    println!("\nFirst 20 sorted elements: {:?}", &data[..20]);
    println!("Last 20 sorted elements: {:?}", &data[data.len() - 20..]);
}
