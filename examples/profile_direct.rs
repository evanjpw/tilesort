use rand::prelude::*;

fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut result = Vec::with_capacity(total_size);
    let mut remaining = total_size;
    let mut tile_idx = 0;

    while remaining > 0 {
        let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);
        let start: i32 = rng.random_range(0..1_000_000);
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

fn main() {
    let mut data = generate_tiled_data(100_000, &[1000]);

    eprintln!("Sorting {} elements...", data.len());
    tilesort::tilesort(&mut data);

    // Verify sorted
    for i in 0..data.len() - 1 {
        assert!(data[i] <= data[i + 1], "Not sorted at index {}", i);
    }
    eprintln!("Sort successful");
}
