// Single line comment

/* Multi-line comment
   on multiple lines */

const x = 5; // Code with trailing comment

/* Inline comment */ const y = 10;

/*
 * JSDoc style comment
 * with multiple lines
 */
function test() {
    return 42;
}

// Expected counts:
// Blank: 2
// Comment: 7
// Code: 5 (line 6 and 8 are code, lines 12-14 are code)
// Total: 14
