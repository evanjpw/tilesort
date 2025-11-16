//! In-place restructuring - clearer version
//!
//! Algorithm:
//! 1. Build mapping of where each tile currently is and where it needs to go
//! 2. For each cycle:
//!    - Save first tile to temp buffer
//!    - Find what's currently at first tile's destination -> move it
//!    - Repeat until we circle back
//!    - Place temp buffer contents

use crate::sorter::constants::{
    MAX_MEDIAN_TILE_SIZE, MAX_NUM_LARGE_TILES, MAX_NUM_TILES, MIN_MEDIAN_TILE_SIZE, MIN_NUM_TILES,
    MIN_TILE_SIZE,
};
use crate::tile_index::TileIndex;
use log::{debug, info};

/// Decide whether to use in-place restructuring based on actual tile distribution.
///
/// Analyzes the complete tile information available from the scan phase to make
/// a cost-based decision:
/// - In-place saves O(n×sizeof(T)) memory but has poor cache locality (random access)
/// - Clone uses O(n×sizeof(T)) extra memory but excellent cache locality (sequential)
///
/// Uses precomputed statistics from TileIndex for zero-cost decision.
pub fn should_use_in_place(tile_index: &TileIndex, n: usize) -> bool {
    let stats = tile_index.stats();

    if stats.num_tiles == 0 {
        return false;
    }

    // Heuristics based on benchmark results:
    // IMPORTANT: Need BOTH many tiles AND large median for in-place to win
    //
    // 1. Many large tiles = in-place wins significantly
    //    (1000 tiles × 1000 elements: 6.3x speedup with in-place!)
    //    (But few large tiles like 54×1852: clone is 10x faster due to better cache)
    if stats.num_tiles > MAX_NUM_LARGE_TILES && stats.median_size >= MAX_MEDIAN_TILE_SIZE {
        debug!(
            "n={}, tiles={}, avg={}, median={}, min={}, decision=INPLACE (many large tiles)",
            n, stats.num_tiles, stats.avg_size, stats.median_size, stats.min_size
        );
        return true;
    }

    // 2. Too many SMALL tiles = cache thrashing (pathological case)
    //    (Random data creates thousands of 1-2 element tiles - clone is better)
    //    Note: This only applies when median is small (checked above)
    if stats.num_tiles > MAX_NUM_TILES && stats.median_size < MIN_MEDIAN_TILE_SIZE {
        debug!(
            "n={}, tiles={}, avg={}, median={}, min={}, decision=CLONE (many tiny tiles)",
            n, stats.num_tiles, stats.avg_size, stats.median_size, stats.min_size
        );
        return false;
    }

    // 3. Very small median tile size = cache thrashing dominates
    //    (Cycle-following through many small tiles has terrible cache behavior)
    if stats.median_size < MIN_MEDIAN_TILE_SIZE {
        debug!(
            "n={}, tiles={}, avg={}, median={}, min={}, decision=CLONE (small median)",
            n, stats.num_tiles, stats.avg_size, stats.median_size, stats.min_size
        );
        return false;
    }

    // 4. Many tiles with tiny minimum = lots of bad cache behavior
    if stats.min_size < MIN_TILE_SIZE && stats.num_tiles > MIN_NUM_TILES {
        debug!(
            "n={}, tiles={}, avg={}, median={}, min={}, decision=CLONE (tiny min, many tiles)",
            n, stats.num_tiles, stats.avg_size, stats.median_size, stats.min_size
        );
        return false;
    }

    // Default: use clone-based approach (safer, better cache behavior)
    // Note: Few large tiles (e.g., 54-100 tiles) still use clone because clone has
    // better sequential memory access patterns. In-place only wins with MANY tiles (>100).
    debug!(
        "n={}, tiles={}, avg={}, median={}, min={}, decision=CLONE (default)",
        n, stats.num_tiles, stats.avg_size, stats.median_size, stats.min_size
    );
    false
}

//restructure_phase_in_place

pub(crate) fn restructure_in_place_permute<T>(data: &mut [T], tile_index: &TileIndex)
where
    T: Clone,
{
    if tile_index.len() <= 1 {
        return;
    }

    // Build element-level permutation: permutation[i] = where element at position i should go
    // This is O(n) space for usize, which is much smaller than O(n) for T
    let n = data.len();
    let mut permutation: Vec<usize> = vec![0; n];

    let mut dest_pos = 0;
    for tile in tile_index.iter() {
        let src_start = tile.start_idx();
        let len = tile.len();

        debug!("Tile: src={} dest={} len={}", src_start, dest_pos, len);

        // Each element in this tile gets mapped to its destination
        for i in 0..len {
            permutation[src_start + i] = dest_pos + i;
        }
        dest_pos += len;
    }

    debug!("Permutation: {:?}", permutation);

    debug!("Starting cycle-following on {} elements", n);

    // Now apply the permutation using standard cycle-following
    let mut visited = vec![false; n];

    for start in 0..n {
        if visited[start] || permutation[start] == start {
            visited[start] = true;
            continue;
        }

        // Save first element in cycle
        let mut temp = data[start].clone();
        visited[start] = true;

        let mut current = start;
        loop {
            let next = permutation[current];

            // Save next's value before overwriting it
            let next_val = data[next].clone();
            visited[next] = true;

            // Write temp to next position in cycle
            data[next] = temp;

            if next == start {
                // Cycle complete
                break;
            }

            // Next iteration: temp becomes next_val
            temp = next_val;
            current = next;
        }
    }
}

/// Phase 2: Use the tile index to reconstruct the sorted array (non-in-place).
/// This uses a full clone but is faster when we've already cloned.
pub fn restructure_in_place_copy<T>(data: &mut [T], tile_index: &TileIndex)
where
    T: Clone,
{
    info!("Restructuring with {} tiles", tile_index.len());

    // Create a copy of the original data
    let original = data.to_vec();

    // Copy tiles in sorted order
    let mut write_pos = 0;
    for (i, tile) in tile_index.iter().enumerate() {
        let start = tile.start_idx();
        let end = start + tile.len();

        debug!(
            "Tile {}: start={}, count={}, copying to position {}",
            i,
            start,
            tile.len(),
            write_pos
        );

        data[write_pos..write_pos + tile.len()].clone_from_slice(&original[start..end]);
        write_pos += tile.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::tilesort;

    #[test]
    fn test_integration() {
        let mut data = vec![3, 4, 5, 1, 2, 6, 7, 8];
        tilesort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
