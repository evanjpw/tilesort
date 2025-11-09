//! Core tilesort algorithm implementation.

use crate::key_extractor::KeyExtractor;
use crate::tile_index::{Tile, TileIndex};
use log::{debug, info};

/// Main tilesort implementation with custom key extraction.
///
/// # Arguments
/// * `data` - The slice to sort
/// * `key_extractor` - Extracts sort keys from elements
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_impl_with_key<T, K, E>(data: &mut [T], key_extractor: E, reverse: bool)
where
    T: Clone,
    K: Ord,
    E: KeyExtractor<T, K>,
{
    if data.len() <= 1 {
        return;
    }

    // Phase 1: Scan and build tile index
    let tile_index = scan_phase(data, key_extractor, reverse);

    // Phase 2: Restructure using the tile index
    restructure_phase(data, &tile_index);
}

/// Main tilesort implementation (no custom key function).
///
/// # Arguments
/// * `data` - The slice to sort
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_impl<T: Ord + Clone>(data: &mut [T], reverse: bool) {
    if data.len() <= 1 {
        return;
    }

    // Phase 1: Scan and build tile index
    let tile_index = scan_phase_without_key(data, reverse);

    // Phase 2: Restructure using the tile index
    restructure_phase(data, &tile_index);
}

fn process_tile_boundaries<K: Ord>(
    tile_index: &mut TileIndex,
    tile_start_idx: &mut Option<usize>,
    idx: usize,
    element_keys: &[K],
    reverse: bool,
) {
    if let Some(start_idx) = tile_start_idx {
        let prev_index: usize = if idx == 0 {
            // First element always starts a new tile
            0
        } else {
            idx - 1
        };

        let prev_key = &element_keys[prev_index];

        // Check if out of order
        let finish_tile = if reverse {
            &element_keys[idx] > prev_key // For descending sort
        } else {
            &element_keys[idx] < prev_key // For ascending sort
        };

        if finish_tile {
            let count = idx - *start_idx;
            let new_tile = Tile::new(*start_idx, count);
            tile_index.insert_tile(new_tile, element_keys, reverse);
            *tile_start_idx = None;
        }
    }

    if tile_start_idx.is_none() {
        *tile_start_idx = Some(idx);
    }
}

fn add_last_tile<K: Ord>(
    tile_index: &mut TileIndex,
    tile_start_idx: &Option<usize>,
    element_keys: &[K],
    reverse: bool,
) {
    let start_idx =
        tile_start_idx.expect("There should be at least one tile index before the end of the data");
    let elements_count = element_keys.len();
    let count = elements_count - start_idx;
    let new_tile = Tile::new(start_idx, count);
    tile_index.insert_tile(new_tile, element_keys, reverse);
}

/// Phase 1: Scan through the data and build the tile index.
fn scan_phase<T, K, E>(data: &[T], key_extractor: E, reverse: bool) -> TileIndex
where
    K: Ord,
    E: KeyExtractor<T, K>,
{
    let mut tile_index = TileIndex::new();
    let mut element_keys: Vec<K> = Vec::with_capacity(data.len());
    let mut tile_start_idx: Option<usize> = None;

    for (idx, element) in data.iter().enumerate() {
        let key = key_extractor.extract_key(element);
        element_keys.push(key);

        process_tile_boundaries(
            &mut tile_index,
            &mut tile_start_idx,
            idx,
            &element_keys,
            reverse,
        );
    }

    // Add the last tile
    add_last_tile(&mut tile_index, &tile_start_idx, &element_keys, reverse);

    tile_index
}

fn scan_phase_without_key<T>(data: &[T], reverse: bool) -> TileIndex
where
    T: Ord,
{
    let mut tile_index = TileIndex::new();
    let mut tile_start_idx: Option<usize> = None;

    for (idx, _) in data.iter().enumerate() {
        process_tile_boundaries(&mut tile_index, &mut tile_start_idx, idx, data, reverse);
    }

    // Add the last tile
    add_last_tile(&mut tile_index, &tile_start_idx, data, reverse);

    tile_index
}

/// Phase 2: Use the tile index to reconstruct the sorted array.
fn restructure_phase<T>(data: &mut [T], tile_index: &TileIndex)
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
