// Simple Rust file for testing
// This file has known line counts

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", "world");

    println!("Hello, world!");
}

// Expected counts:
// Blank: 2
// Comment: 4
// Code: 6
// Total: 12
