//! Core tilesort algorithm implementation.

use crate::key_extractor::KeyExtractor;
use restructure::{restructure_in_place_copy, restructure_in_place_permute, should_use_in_place};
use scan::{scan_phase, scan_phase_without_key};

mod constants;
mod restructure;
mod scan;

/// In-place tilesort implementation with custom key extraction (no extra clone).
///
/// # Arguments
/// * `data` - The slice to sort in-place
/// * `key_extractor` - Extracts sort keys from elements
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_impl_with_key_inplace<T, K, E>(
    data: &mut [T],
    key_extractor: E,
    reverse: bool,
) where
    T: Clone,
    K: Ord,
    E: KeyExtractor<T, K>,
{
    if data.len() <= 1 {
        return;
    }

    // Phase 1: Scan and build tile index
    let mut tile_index = scan_phase(data, key_extractor, reverse);

    // Finalize statistics for heuristic decision
    tile_index.finalize_statistics();

    // Phase 2: Choose restructure strategy based on tile characteristics
    if should_use_in_place(&tile_index, data.len()) {
        restructure_in_place_permute(data, &tile_index);
    } else {
        restructure_in_place_copy(data, &tile_index);
    }
}

/// In-place tilesort implementation (no custom key function, no extra clone).
///
/// # Arguments
/// * `data` - The slice to sort in-place
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_impl_inplace<T: Ord + Clone>(data: &mut [T], reverse: bool) {
    if data.len() <= 1 {
        return;
    }

    // Phase 1: Scan and build tile index
    let mut tile_index = scan_phase_without_key(data, reverse);

    // Finalize statistics for heuristic decision
    tile_index.finalize_statistics();

    // Phase 2: Choose restructure strategy based on tile characteristics
    if should_use_in_place(&tile_index, data.len()) {
        restructure_in_place_permute(data, &tile_index);
    } else {
        restructure_in_place_copy(data, &tile_index);
    }
}

/// Copy-based tilesort (no custom key function). Returns a sorted copy.
/// Performs exactly ONE clone - directly into sorted order.
///
/// # Arguments
/// * `data` - The slice to read (not modified)
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_copy<T: Ord + Clone>(data: &[T], reverse: bool) -> Vec<T> {
    if data.len() <= 1 {
        return data.to_vec();
    }

    // Phase 1: Scan and build tile index
    let tile_index = scan_phase_without_key(data, reverse);

    // Phase 2: Clone directly into sorted order (single allocation, single copy)
    let mut result = Vec::with_capacity(data.len());
    for tile in tile_index.iter() {
        let start = tile.start_idx();
        let end = start + tile.len();
        result.extend_from_slice(&data[start..end]);
    }
    result
}

/// Copy-based tilesort with custom key extraction. Returns a sorted copy.
/// Performs exactly ONE clone - directly into sorted order.
///
/// # Arguments
/// * `data` - The slice to read (not modified)
/// * `key_extractor` - Extracts sort keys from elements
/// * `reverse` - If true, sort in descending order; if false, ascending
pub(crate) fn tilesort_copy_with_key<T, K, E>(data: &[T], key_extractor: E, reverse: bool) -> Vec<T>
where
    T: Clone,
    K: Ord,
    E: KeyExtractor<T, K>,
{
    if data.len() <= 1 {
        return data.to_vec();
    }

    // Phase 1: Scan and build tile index
    let tile_index = scan_phase(data, key_extractor, reverse);

    // Phase 2: Clone directly into sorted order (single allocation, single copy)
    let mut result = Vec::with_capacity(data.len());
    for tile in tile_index.iter() {
        let start = tile.start_idx();
        let end = start + tile.len();
        result.extend_from_slice(&data[start..end]);
    }
    result
}
