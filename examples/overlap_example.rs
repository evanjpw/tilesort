fn main() {
    // Tile A: [102, 103, 104, 105, 106] at positions 0-4
    // Tile B: [100, 101, 102, 103, 104] at positions 5-9
    // Expected: [100, 101, 102, 102, 103, 103, 104, 104, 105, 106]
    let mut data = vec![102, 103, 104, 105, 106, 100, 101, 102, 103, 104];

    println!("\n=== SIMPLE OVERLAP TEST ===");
    println!("Input:");
    println!("  Tile A (indices 0-4): {:?}", &data[0..5]);
    println!("  Tile B (indices 5-9): {:?}", &data[5..10]);
    println!("\nBefore sort: {:?}", data);

    tilesort::tilesort(&mut data);

    println!("After sort:  {:?}", data);
    println!("Expected:    [100, 101, 102, 102, 103, 103, 104, 104, 105, 106]");

    // Verify
    let mut errors = 0;
    for i in 0..data.len() - 1 {
        if data[i] > data[i + 1] {
            println!("ERROR at index {}: {} > {}", i, data[i], data[i + 1]);
            errors += 1;
        }
    }

    if errors == 0 {
        println!("\n✓ Sort successful!");
    } else {
        println!("\n✗ Sort FAILED with {} errors", errors);
    }
}
