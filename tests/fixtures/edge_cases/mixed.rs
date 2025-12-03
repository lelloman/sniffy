// Comment at start
fn calculate(x: i32) -> i32 { // Inline comment

    /* Block comment */ let y = x * 2; // More inline

    y + 1 // Result
} // End function

// Expected counts:
// Blank: 2
// Comment: 2 (lines 1 and 9)
// Code: 5 (lines 2, 4, 6, 7 all have code)
// Total: 9
