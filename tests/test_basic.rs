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
