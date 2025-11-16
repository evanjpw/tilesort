use crate::tile_index::{Tile, TileIndex};
use crate::KeyExtractor;

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
pub fn scan_phase<T, K, E>(data: &[T], key_extractor: E, reverse: bool) -> TileIndex
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

pub fn scan_phase_without_key<T>(data: &[T], reverse: bool) -> TileIndex
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
