use rand::prelude::*;
use tilesort::tilesort;

fn generate_minimally_overlapping_tiles(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let data: Vec<i32> = (0..total_size as i32).collect();

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

    tiles.shuffle(&mut rng);
    tiles.into_iter().flatten().collect()
}

fn main() {
    let tile_sizes = vec![8000, 10000, 12000, 9000, 11000];

    // Run multiple times for profiling
    for _ in 0..100 {
        let mut data = generate_minimally_overlapping_tiles(1_000_000, &tile_sizes);
        tilesort(&mut data);
    }
}
