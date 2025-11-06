use log::debug;

/// Represents a contiguous sorted block (tile) in the input data.
#[derive(Debug, Clone)]
pub struct Tile<K> {
    /// Starting index in the original array
    start_index: usize,
    /// Number of elements in this tile
    count: usize,
    /// Key of the first element (the "tile key")
    tile_key: K,
    /// Key of the last element (for range checking)
    end_key: K,
}

impl<K: Ord + Clone> Tile<K> {
    pub(crate) fn new(start_index: usize, count: usize, tile_key: K, end_key: K) -> Self {
        Tile {
            start_index,
            count,
            tile_key,
            end_key,
        }
    }

    pub(crate) fn start_idx(&self) -> usize {
        self.start_index
    }

    fn end_idx(&self) -> usize {
        self.start_index + self.count
    }

    #[allow(dead_code)]
    fn end_index(&self) -> usize {
        self.end_idx() - 1
    }

    pub(crate) fn len(&self) -> usize {
        self.count
    }

    /// Binary search to find the split point in a tile.
    pub(crate) fn find_split_point(
        &self,
        element_keys: &[K],
        split_key: &K,
        reverse: bool,
    ) -> usize {
        let start = self.start_index;
        let end = self.start_index + self.count;
        let slice = &element_keys[start..end];
        let result = slice.binary_search_by(|elem| {
            if reverse {
                split_key.cmp(elem)
            } else {
                elem.cmp(split_key)
            }
        });

        match result {
            Ok(idx) => start + idx,
            Err(idx) => start + idx,
        }
    }
}

/// A collection of tiles maintained in sorted order by tile key.
///
/// This is a newtype wrapper around Vec<Tile> to allow easy replacement
/// with a different data structure if needed.
#[derive(Debug)]
pub struct TileIndex<K> {
    tiles: Vec<Tile<K>>,
}

impl<K: Ord + Clone + std::fmt::Debug> TileIndex<K> {
    pub(crate) fn new() -> Self {
        TileIndex { tiles: Vec::new() }
    }

    pub(crate) fn len(&self) -> usize {
        self.tiles.len()
    }

    fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    fn get(&self, index: usize) -> Option<&Tile<K>> {
        self.tiles.get(index)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Tile<K>> {
        self.tiles.iter()
    }

    fn insert(&mut self, index: usize, tile: Tile<K>) {
        self.tiles.insert(index, tile);
    }

    fn push(&mut self, tile: Tile<K>) {
        self.tiles.push(tile);
    }

    /// Insert a new tile into the tile index, potentially splitting the new tile if it spans multiple positions.
    pub fn insert_tile(&mut self, new_tile: Tile<K>, element_keys: &[K], reverse: bool) {
        // If this is the first tile, just add it
        if self.is_empty() {
            self.push(new_tile);
            return;
        }

        // Find where the new tile's start (tile_key) should be inserted
        // Also check for overlaps with existing tiles
        let mut insert_position = self.len(); // Default to end

        for i in 0..self.len() {
            let current = self.get(i).unwrap();

            let should_insert_before = if reverse {
                new_tile.tile_key > current.tile_key
            } else {
                new_tile.tile_key < current.tile_key
            };

            if should_insert_before {
                insert_position = i;
                break;
            }

            // Check if the new tile falls within this existing tile's range
            // This means we need to split the EXISTING tile
            let new_within_existing = if reverse {
                new_tile.tile_key < current.tile_key && new_tile.tile_key > current.end_key
            } else {
                new_tile.tile_key > current.tile_key && new_tile.tile_key < current.end_key
            };

            if new_within_existing {
                debug!(
                    "New tile falls within existing tile at position {}, splitting existing",
                    i
                );
                self.split_existing_and_insert(i, new_tile, element_keys, reverse);
                return;
            }
        }

        // Check if the new tile's range extends beyond where it should fit
        // This means we need to split the NEW tile
        for i in insert_position..self.len() {
            let existing = self.get(i).unwrap();

            // Check if the new tile's end_key extends past this existing tile's start
            let overlaps = if reverse {
                new_tile.end_key < existing.tile_key
            } else {
                new_tile.end_key > existing.tile_key
            };

            if overlaps {
                // The new tile spans multiple positions - we need to split it
                debug!("New tile spans multiple positions, splitting new tile");
                self.split_new_tile_and_insert(new_tile, element_keys, insert_position, i, reverse);
                return;
            }
        }

        // No conflict, insert normally
        self.insert(insert_position, new_tile);
    }

    /// Split the new tile at the boundary and recursively insert pieces.
    fn split_new_tile_and_insert(
        &mut self,
        new_tile: Tile<K>,
        element_keys: &[K],
        insert_position: usize,
        overlapping_tile_index: usize,
        reverse: bool,
    ) {
        // Find the split point - where does the overlapping tile's range begin?
        let overlapping_tile = self.get(overlapping_tile_index).unwrap();
        let split_key = &overlapping_tile.tile_key;

        debug!(
            "Splitting new tile at start={}, count={} at split_key={:?}",
            new_tile.start_index, new_tile.count, split_key
        );

        // Find where in the new tile we should split
        let split_point = new_tile.find_split_point(element_keys, split_key, reverse);

        debug!("Split point: {}", split_point);

        // Create the two pieces
        if split_point == new_tile.start_index {
            // Split point is at the start - shouldn't happen, but handle gracefully
            debug!("Split point at start, inserting whole tile");
            self.insert(insert_position, new_tile);
            return;
        }

        if split_point >= new_tile.end_idx() {
            // Split point is beyond the end - shouldn't happen, but handle gracefully
            debug!("Split point beyond end, inserting whole tile");
            self.insert(insert_position, new_tile);
            return;
        }

        let first_piece = Tile {
            start_index: new_tile.start_index,
            count: split_point - new_tile.start_index,
            tile_key: new_tile.tile_key.clone(),
            end_key: element_keys[split_point - 1].clone(),
        };

        let second_piece = Tile {
            start_index: split_point,
            count: (new_tile.start_index + new_tile.count) - split_point,
            tile_key: element_keys[split_point].clone(),
            end_key: new_tile.end_key.clone(),
        };

        debug!(
            "Split into: piece1(start={}, count={}), piece2(start={}, count={})",
            first_piece.start_index,
            first_piece.count,
            second_piece.start_index,
            second_piece.count
        );

        // Insert the first piece at the current position
        self.insert(insert_position, first_piece);

        // Recursively insert the second piece
        self.insert_tile(second_piece, element_keys, reverse);
    }

    /// Split an existing tile and insert the new tile between the pieces.
    fn split_existing_and_insert(
        &mut self,
        tile_idx: usize,
        new_tile: Tile<K>,
        element_keys: &[K],
        reverse: bool,
    ) {
        // Get the tile to split (need to clone it as we'll be modifying the index)
        let original_tile = self.get(tile_idx).unwrap().clone();

        debug!(
            "Splitting existing tile at idx={}, start={}, count={}",
            tile_idx, original_tile.start_index, original_tile.count
        );

        // Find where to split the existing tile (at the new tile's start key)
        let split_point = original_tile.find_split_point(element_keys, &new_tile.tile_key, reverse);

        debug!("Split point: {}", split_point);

        if split_point == original_tile.start_index || split_point >= original_tile.end_idx() {
            // Shouldn't happen, but handle gracefully
            debug!("Invalid split point, inserting without splitting");
            return;
        }

        // Create the two pieces of the existing tile
        let first_piece = Tile {
            start_index: original_tile.start_index,
            count: split_point - original_tile.start_index,
            tile_key: original_tile.tile_key.clone(),
            end_key: element_keys[split_point - 1].clone(),
        };

        let second_piece = Tile {
            start_index: split_point,
            count: (original_tile.start_index + original_tile.count) - split_point,
            tile_key: element_keys[split_point].clone(),
            end_key: original_tile.end_key.clone(),
        };

        // Remove the original tile
        self.tiles.remove(tile_idx);

        // Insert the first piece at the original position
        self.insert(tile_idx, first_piece);

        // Recursively insert the new tile (might need further splitting)
        self.insert_tile(new_tile, element_keys, reverse);

        // Recursively insert the second piece
        self.insert_tile(second_piece, element_keys, reverse);
    }
}
