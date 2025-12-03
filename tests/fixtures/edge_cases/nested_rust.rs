/* Rust supports truly nested comments! */

fn main() {
    /* Outer comment
       /* This is nested inside */
       Still in outer comment
    */
    let x = 5; // After nested comment block
}

// Expected counts:
// Blank: 1
// Comment: 6 (lines 1, 4-7, line 11 onwards)
// Code: 3 (lines 3, 8, 9)
// Total: 10
