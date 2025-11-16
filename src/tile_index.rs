use log::debug;

/// Statistics about tile size distribution (precomputed during scan phase)
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TileStats {
    pub num_tiles: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub median_size: usize,
    pub avg_size: usize,
    pub total_elements: usize,
}

/// Represents a contiguous sorted block (tile) in the input data.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Starting index in the original array
    start_index: usize,
    /// Number of elements in this tile
    count: usize,
}

impl Tile {
    pub(crate) fn new(start_index: usize, count: usize) -> Self {
        Tile { start_index, count }
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

    /// Get the key of the first element (the "tile key")
    pub(crate) fn tile_key<'a, K>(&self, element_keys: &'a [K]) -> &'a K {
        &element_keys[self.start_index]
    }

    /// Get the key of the last element (for range checking)
    pub(crate) fn end_key<'a, K>(&self, element_keys: &'a [K]) -> &'a K {
        &element_keys[self.start_index + self.count - 1]
    }

    /// Binary search to find the split point in a tile.
    pub(crate) fn find_split_point<K: Ord>(
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
///
/// Tracks tile size statistics during construction for zero-cost heuristic decisions.
#[derive(Debug)]
pub struct TileIndex {
    tiles: Vec<Tile>,
    /// Minimum tile size (updated as tiles are added)
    min_tile_size: usize,
    /// Maximum tile size (updated as tiles are added)
    max_tile_size: usize,
    /// Total elements across all tiles
    total_elements: usize,
    /// Median tile size (computed once at finalization via counting sort)
    median_tile_size: Option<usize>,
}

impl TileIndex {
    pub(crate) fn new() -> Self {
        TileIndex {
            tiles: Vec::new(),
            min_tile_size: usize::MAX,
            max_tile_size: 0,
            total_elements: 0,
            median_tile_size: None,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tiles.len()
    }

    fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    fn get(&self, index: usize) -> Option<&Tile> {
        self.tiles.get(index)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    fn insert(&mut self, index: usize, tile: Tile) {
        let tile_len = tile.len();
        self.min_tile_size = self.min_tile_size.min(tile_len);
        self.max_tile_size = self.max_tile_size.max(tile_len);
        self.total_elements += tile_len;
        self.tiles.insert(index, tile);
    }

    fn remove(&mut self, index: usize) -> Tile {
        let tile = self.tiles.remove(index);
        self.total_elements -= tile.len();
        // Note: min/max are not adjusted as recomputing would be expensive
        // They remain conservative upper/lower bounds
        tile
    }

    fn push(&mut self, tile: Tile) {
        let tile_len = tile.len();
        self.min_tile_size = self.min_tile_size.min(tile_len);
        self.max_tile_size = self.max_tile_size.max(tile_len);
        self.total_elements += tile_len;
        self.tiles.push(tile);
    }

    // TODO: We could optimize further by only checking if median is above/below specific
    //  thresholds (750, 100) rather than computing the exact median. This would be O(num_tiles)
    //  time with O(1) space by counting tiles >= threshold vs < threshold. Since current
    //  overhead is ~1µs, this optimization is deferred.
    /// Compute median tile size using counting sort approach.
    /// Called once after all tiles have been added.
    ///
    /// Optimized to avoid allocation for small tile counts or when tiles are large.
    ///
    pub(crate) fn finalize_statistics(&mut self) {
        if self.tiles.is_empty() {
            return;
        }

        let num_tiles = self.tiles.len();

        // For small numbers of tiles, just collect and use nth_element (no full sort needed)
        // This avoids allocating large counting arrays when tiles are big
        if num_tiles <= 100 || self.max_tile_size > 10000 {
            let mut sizes: Vec<usize> = self.tiles.iter().map(|t| t.len()).collect();
            let median_idx = sizes.len() / 2;

            // Use select_nth_unstable to find median in O(n) average time, no allocation beyond sizes vec
            let (_, median, _) = sizes.select_nth_unstable(median_idx);
            self.median_tile_size = Some(*median);
            return;
        }

        // For many small-ish tiles, use counting sort (O(max_tile_size) space)
        let mut size_counts = vec![0usize; self.max_tile_size + 1];

        for tile in &self.tiles {
            size_counts[tile.len()] += 1;
        }

        // Find median by walking through counts
        let median_pos = num_tiles / 2;
        let mut cumulative = 0;

        for (size, &count) in size_counts.iter().enumerate() {
            cumulative += count;
            if cumulative > median_pos {
                self.median_tile_size = Some(size);
                break;
            }
        }
    }

    /// Get tile size statistics (for heuristic decisions)
    pub(crate) fn stats(&self) -> TileStats {
        TileStats {
            num_tiles: self.tiles.len(),
            min_size: if self.tiles.is_empty() {
                0
            } else {
                self.min_tile_size
            },
            max_size: self.max_tile_size,
            median_size: self.median_tile_size.unwrap_or(0),
            avg_size: if self.tiles.is_empty() {
                0
            } else {
                self.total_elements / self.tiles.len()
            },
            total_elements: self.total_elements,
        }
    }

    /// Insert a new tile into the tile index, potentially splitting the new tile if it spans multiple positions.
    /// Uses iterative approach with work queue to avoid stack overflow on deep recursion.
    pub fn insert_tile<K: Ord>(&mut self, new_tile: Tile, element_keys: &[K], reverse: bool) {
        let mut work_queue = vec![new_tile];

        while let Some(tile_to_insert) = work_queue.pop() {
            self.insert_tile_single(&mut work_queue, tile_to_insert, element_keys, reverse);
        }
    }

    /// Insert a single tile, potentially adding more tiles to the work queue if splitting is needed.
    fn insert_tile_single<K: Ord>(
        &mut self,
        work_queue: &mut Vec<Tile>,
        new_tile: Tile,
        element_keys: &[K],
        reverse: bool,
    ) {
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
                new_tile.tile_key(element_keys) > current.tile_key(element_keys)
            } else {
                new_tile.tile_key(element_keys) < current.tile_key(element_keys)
            };

            if should_insert_before {
                insert_position = i;
                break;
            }

            // Check if the new tile falls within this existing tile's range
            // This means we need to split the EXISTING tile
            let new_within_existing = if reverse {
                new_tile.tile_key(element_keys) <= current.tile_key(element_keys)
                    && new_tile.tile_key(element_keys) >= current.end_key(element_keys)
            } else {
                new_tile.tile_key(element_keys) >= current.tile_key(element_keys)
                    && new_tile.tile_key(element_keys) <= current.end_key(element_keys)
            };

            if new_within_existing {
                // Don't split single-element tiles - just continue to find the right position
                if current.len() == 1 {
                    debug!(
                        "New tile matches single-element tile at position {}, continuing",
                        i
                    );
                    continue;
                }
                debug!(
                    "New tile falls within existing tile at position {}, splitting existing",
                    i
                );
                self.split_existing_and_insert(work_queue, i, new_tile, element_keys, reverse);
                return;
            }
        }

        // Check if the new tile's range extends beyond where it should fit
        // This means we need to split the NEW tile
        for i in insert_position..self.len() {
            let existing = self.get(i).unwrap();

            // Check if the new tile's end_key extends past this existing tile's start
            let overlaps = if reverse {
                new_tile.end_key(element_keys) <= existing.tile_key(element_keys)
            } else {
                new_tile.end_key(element_keys) >= existing.tile_key(element_keys)
            };

            if overlaps {
                // The new tile spans multiple positions - we need to split it
                debug!("New tile spans multiple positions, splitting new tile");
                self.split_new_tile_and_insert(
                    work_queue,
                    new_tile,
                    element_keys,
                    insert_position,
                    i,
                    reverse,
                );
                return;
            }
        }

        // No conflict, insert normally
        self.insert(insert_position, new_tile);
    }

    /// Split the new tile at the boundary and add pieces to work queue.
    fn split_new_tile_and_insert<K: Ord>(
        &mut self,
        work_queue: &mut Vec<Tile>,
        new_tile: Tile,
        element_keys: &[K],
        insert_position: usize,
        overlapping_tile_index: usize,
        reverse: bool,
    ) {
        // Find the split point - where does the overlapping tile's range begin?
        let overlapping_tile = self.get(overlapping_tile_index).unwrap();
        let split_key = overlapping_tile.tile_key(element_keys);

        debug!(
            "Splitting new tile at start={}, count={}",
            new_tile.start_idx(),
            new_tile.len()
        );

        // Find where in the new tile we should split
        let split_point = new_tile.find_split_point(element_keys, split_key, reverse);

        debug!("Split point: {}", split_point);

        // Create the two pieces
        if split_point == new_tile.start_idx() {
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

        let first_piece = Tile::new(new_tile.start_idx(), split_point - new_tile.start_idx());

        let second_piece = Tile::new(
            split_point,
            (new_tile.start_idx() + new_tile.len()) - split_point,
        );

        debug!(
            "Split into: piece1(start={}, count={}), piece2(start={}, count={})",
            first_piece.start_idx(),
            first_piece.len(),
            second_piece.start_idx(),
            second_piece.len()
        );

        // Insert the first piece at the current position
        self.insert(insert_position, first_piece);

        // Add the second piece to work queue for later insertion
        work_queue.push(second_piece);
    }

    /// Split an existing tile and add pieces to work queue.
    fn split_existing_and_insert<K: Ord>(
        &mut self,
        work_queue: &mut Vec<Tile>,
        tile_idx: usize,
        new_tile: Tile,
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
        let split_point =
            original_tile.find_split_point(element_keys, new_tile.tile_key(element_keys), reverse);

        debug!("Split point: {}", split_point);

        if split_point >= original_tile.end_idx() {
            // Split point is beyond the end - shouldn't happen
            debug!("Invalid split point (beyond end), inserting without splitting");
            return;
        }

        // Handle the case where split point equals start (both tiles start with same value)
        // In this case, split off a single element from the front
        let first_piece_size = if split_point == original_tile.start_index {
            debug!("Split point at start, splitting off single element");
            1
        } else {
            split_point - original_tile.start_index
        };

        // Create the two pieces of the existing tile
        let first_piece = Tile::new(original_tile.start_index, first_piece_size);

        let second_piece = Tile::new(
            original_tile.start_index + first_piece_size,
            original_tile.count - first_piece_size,
        );

        // Remove the original tile (adjusts statistics)
        self.remove(tile_idx);

        // Insert the first piece at the original position
        self.insert(tile_idx, first_piece);

        // Add tiles to work queue for later insertion (order matters: second piece first, then new tile)
        work_queue.push(second_piece);
        work_queue.push(new_tile);
    }
}
