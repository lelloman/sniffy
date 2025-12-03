/*
 * This is a very long multi-line comment
 * that spans many lines
 * to test the state machine
 * properly tracks comment blocks
 * across multiple lines
 * of text
 * in the source file
 */

fn main() {
    println!("Hello");
}

/* Another multi-line
   comment here
   spanning several
   lines */

// Expected counts:
// Blank: 2
// Comment: 13
// Code: 3
// Total: 18
