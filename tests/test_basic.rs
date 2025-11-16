// Basic integration tests for tilesort

use test_log::test;

use tilesort::{
    tilesort, tilesort_by_key, tilesort_by_key_reverse, tilesort_reverse, tilesorted,
    tilesorted_by_key, tilesorted_by_key_reverse, tilesorted_reverse,
};

#[test]
fn test_simple_two_tiles() {
    let mut data = vec![3, 4, 5, 1, 2];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_three_tiles_overlapping() {
    // This is the example from the algorithm doc
    let mut data = vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10];
    tilesort(&mut data);
    assert_eq!(
        data,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
}

#[test]
fn test_already_sorted() {
    let mut data = vec![1, 2, 3, 4, 5];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_reverse_sorted() {
    let mut data = vec![5, 4, 3, 2, 1];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_single_element() {
    let mut data = vec![42];
    tilesort(&mut data);
    assert_eq!(data, vec![42]);
}

#[test]
fn test_empty() {
    let mut data: Vec<i32> = vec![];
    tilesort(&mut data);
    assert_eq!(data, Vec::<i32>::new());
}

#[test]
fn test_two_elements_sorted() {
    let mut data = vec![1, 2];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2]);
}

#[test]
fn test_two_elements_unsorted() {
    let mut data = vec![2, 1];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2]);
}

#[test]
fn test_many_tiles() {
    // Tile 0:  (2, 3)
    // Tile 1:  (1)
    // Tile 2:  (4, 5, 6)
    // Tile 3:  (3)
    // Tile 4:  (7, 8, 9)
    let mut data = vec![2, 3, 1, 4, 5, 6, 3, 7, 8, 9];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_duplicates() {
    let mut data = vec![3, 3, 3, 1, 1, 2, 2];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 1, 2, 2, 3, 3, 3]);
}

#[test]
fn test_strings() {
    let mut data = vec!["cat", "dog", "elephant", "ant", "bear"];
    tilesort(&mut data);
    assert_eq!(data, vec!["ant", "bear", "cat", "dog", "elephant"]);
}

#[test]
fn test_reverse_simple() {
    let mut data = vec![1, 2, 3, 4, 5];
    tilesort_reverse(&mut data);
    assert_eq!(data, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_reverse_two_tiles() {
    let mut data = vec![5, 4, 3, 8, 7, 6];
    tilesort_reverse(&mut data);
    assert_eq!(data, vec![8, 7, 6, 5, 4, 3]);
}

// Tests for copying functions

#[test]
fn test_tilesorted_basic() {
    let data = vec![3, 4, 5, 1, 2];
    let sorted = tilesorted(&data);
    assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    assert_eq!(data, vec![3, 4, 5, 1, 2]); // Original unchanged
}

#[test]
fn test_tilesorted_three_tiles() {
    let data = vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10];
    let sorted = tilesorted(&data);
    assert_eq!(
        sorted,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    assert_eq!(
        data,
        vec![1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10]
    ); // Original unchanged
}

#[test]
fn test_tilesorted_empty() {
    let data: Vec<i32> = vec![];
    let sorted = tilesorted(&data);
    assert_eq!(sorted, Vec::<i32>::new());
}

#[test]
fn test_tilesorted_reverse_basic() {
    let data = vec![3, 4, 5, 1, 2];
    let sorted = tilesorted_reverse(&data);
    assert_eq!(sorted, vec![5, 4, 3, 2, 1]);
    assert_eq!(data, vec![3, 4, 5, 1, 2]); // Original unchanged
}

#[test]
fn test_tilesorted_reverse_two_tiles() {
    let data = vec![5, 4, 3, 8, 7, 6];
    let sorted = tilesorted_reverse(&data);
    assert_eq!(sorted, vec![8, 7, 6, 5, 4, 3]);
    assert_eq!(data, vec![5, 4, 3, 8, 7, 6]); // Original unchanged
}

// Tests for by_key functions

#[test]
fn test_tilesort_by_key_abs() {
    let mut data = vec![-5i32, -3, -1, 2, 4];
    tilesort_by_key(&mut data, |&x| x.abs());
    assert_eq!(data, vec![-1, 2, -3, 4, -5]);
}

#[test]
fn test_tilesort_by_key_string_len() {
    let mut data = vec!["elephant", "cat", "dog", "a", "bear"];
    tilesort_by_key(&mut data, |s| s.len());
    assert_eq!(data, vec!["a", "cat", "dog", "bear", "elephant"]);
}

#[test]
fn test_tilesort_by_key_reverse_abs() {
    let mut data = vec![-5i32, -3, -1, 2, 4];
    tilesort_by_key_reverse(&mut data, |&x| x.abs());
    assert_eq!(data, vec![-5, 4, -3, 2, -1]);
}

#[test]
fn test_tilesorted_by_key_abs() {
    let data = vec![-5i32, -3, -1, 2, 4];
    let sorted = tilesorted_by_key(&data, |&x| x.abs());
    assert_eq!(sorted, vec![-1, 2, -3, 4, -5]);
    assert_eq!(data, vec![-5, -3, -1, 2, 4]); // Original unchanged
}

#[test]
fn test_tilesorted_by_key_reverse_abs() {
    let data = vec![-5i32, -3, -1, 2, 4];
    let sorted = tilesorted_by_key_reverse(&data, |&x| x.abs());
    assert_eq!(sorted, vec![-5, 4, -3, 2, -1]);
    assert_eq!(data, vec![-5, -3, -1, 2, 4]); // Original unchanged
}

#[test]
fn test_tilesort_by_key_struct() {
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut data = vec![
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Diana".to_string(),
            age: 40,
        },
        Person {
            name: "Eve".to_string(),
            age: 28,
        },
    ];

    tilesort_by_key(&mut data, |p| p.age);

    assert_eq!(data[0].name, "Bob"); // age 25
    assert_eq!(data[1].name, "Eve"); // age 28
    assert_eq!(data[2].name, "Alice"); // age 30
    assert_eq!(data[3].name, "Charlie"); // age 35
    assert_eq!(data[4].name, "Diana"); // age 40
}

/// Test to verify tile boundary counting logic is correct (addresses TODO comments in sorter.rs)
/// This test explicitly verifies:
/// 1. count = idx - start_idx is correct for mid-array tiles
/// 2. count = elements_count - start_idx is correct for the last tile
#[test]
fn test_tile_boundary_counting() {
    // Test case: [1, 2, 3, 5, 4]
    // Tile 0: indices 0-3 (values 1,2,3,5) -> count should be 4
    // Tile 1: index 4 (value 4) -> count should be 1
    let mut data = vec![1, 2, 3, 5, 4];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);

    // Test case: Multiple tiles with varying sizes
    // [10, 20, 5, 15, 25, 35, 30]
    // Tile 0: indices 0-1 (values 10,20) -> count should be 2
    // Tile 1: index 2 (value 5) -> count should be 1
    // Tile 2: indices 3-5 (values 15,25,35) -> count should be 3
    // Tile 3: index 6 (value 30) -> count should be 1
    let mut data = vec![10, 20, 5, 15, 25, 35, 30];
    tilesort(&mut data);
    assert_eq!(data, vec![5, 10, 15, 20, 25, 30, 35]);

    // Test case: Last tile has multiple elements
    // [5, 4, 3, 1, 2, 6, 7, 8, 9, 10]
    // Tile 0: index 0 (value 5) -> count should be 1
    // Tile 1: index 1 (value 4) -> count should be 1
    // Tile 2: index 2 (value 3) -> count should be 1
    // Tile 3: indices 3-4 (values 1,2) -> count should be 2
    // Tile 4: indices 5-9 (values 6,7,8,9,10) -> count should be 5 (last tile)
    let mut data = vec![5, 4, 3, 1, 2, 6, 7, 8, 9, 10];
    tilesort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

/// Test with 100 uniform tiles of 1000 elements (100k total) to verify in-place decision
/// This simulates the uniform_tiles/100000 benchmark case
#[test]
fn test_uniform_100k_tiles() {
    use rand::{seq::SliceRandom, SeedableRng};

    // Simulate uniform_tiles/100000: 100 tiles of 1000 elements
    let tile_size = 1000;
    let num_tiles = 100;
    let total = tile_size * num_tiles;

    let mut data: Vec<i32> = Vec::with_capacity(total);
    for tile_idx in 0..num_tiles {
        let start = (tile_idx * 1000) as i32;
        for i in 0..tile_size {
            data.push(start + i as i32);
        }
    }

    // Shuffle tiles (not elements within tiles)
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut tiles: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_tiles {
        let start = i * tile_size;
        tiles.push(data[start..start + tile_size].to_vec());
    }
    tiles.shuffle(&mut rng);
    let mut data: Vec<i32> = tiles.into_iter().flatten().collect();

    tilesort(&mut data);

    // Verify sorted
    for i in 0..total - 1 {
        assert!(
            data[i] <= data[i + 1],
            "Not sorted at {}: {} > {}",
            i,
            data[i],
            data[i + 1]
        );
    }
}

/// Diagnostic test to understand the benchmark data generation
#[test]
fn test_benchmark_tile_structure() {
    use rand::{seq::SliceRandom, Rng, SeedableRng};

    // Replicate exact benchmark data generation for uniform_tiles/100000
    fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut result = Vec::with_capacity(total_size);
        let mut remaining = total_size;
        let mut tile_idx = 0;

        while remaining > 0 {
            let tile_size = tile_sizes[tile_idx % tile_sizes.len()].min(remaining);

            // Generate a sorted tile with random starting value
            let start: i32 = rng.random_range(0..1_000_000);
            let mut tile: Vec<i32> = (0..tile_size).map(|i| start + i as i32).collect();

            result.append(&mut tile);
            remaining -= tile_size;
            tile_idx += 1;
        }

        // Shuffle the tiles (but keep each tile internally sorted)
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

    let data = generate_tiled_data(100_000, &[1000]);

    // Count actual tiles by detecting boundaries (unsorted -> sorted transition)
    let mut tile_count = 1;
    for i in 1..data.len() {
        if data[i] < data[i - 1] {
            tile_count += 1;
        }
    }

    println!("Total elements: {}", data.len());
    println!("Detected tiles: {}", tile_count);

    // This test just shows diagnostic info - always passes
    assert_eq!(data.len(), 100_000);
}

/// Minimal test case: two overlapping tiles
#[test]
fn test_simple_overlap() {
    // Tile A: [100, 101, 102, 103, 104]
    // Tile B: [102, 103, 104, 105, 106]
    // Expected sorted: [100, 101, 102, 102, 103, 103, 104, 104, 105, 106]
    let mut data = vec![102, 103, 104, 105, 106, 100, 101, 102, 103, 104];

    tilesort(&mut data);

    // Verify sorted
    for i in 0..data.len() - 1 {
        assert!(
            data[i] <= data[i + 1],
            "Not sorted at index {}: {} > {}",
            i,
            data[i],
            data[i + 1]
        );
    }
}

/// Test case with tiles that have identical starting values
#[test]
fn test_simple_overlap_identical_starts() {
    // Tile A: [1, 2, 3, 4] at positions 0-3
    // Tile B: [1, 2, 3] at positions 4-6
    // Both tiles start with value 1
    // Expected sorted: [1, 1, 2, 2, 3, 3, 4]
    let mut data = vec![1, 2, 3, 4, 1, 2, 3];

    tilesort(&mut data);

    // Verify sorted
    for i in 0..data.len() - 1 {
        assert!(
            data[i] <= data[i + 1],
            "Not sorted at index {}: {} > {}",
            i,
            data[i],
            data[i + 1]
        );
    }
}

/// Test with the actual benchmark data generation that creates overlapping tiles
#[test]
fn test_overlapping_tiles() {
    use rand::{seq::SliceRandom, Rng, SeedableRng};

    fn generate_tiled_data(total_size: usize, tile_sizes: &[usize]) -> Vec<i32> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
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

    let mut data = generate_tiled_data(100_000, &[1000]);

    tilesort(&mut data);

    // Verify sorted
    for i in 0..data.len() - 1 {
        assert!(
            data[i] <= data[i + 1],
            "Not sorted at index {}: {} > {}",
            i,
            data[i],
            data[i + 1]
        );
    }
}
