/* This is C code that tests nested comments */
/* Outer comment /* This won't nest in C */ still comment */

int main() {
    /* Another comment */
    return 0;
}

// Note: C doesn't support truly nested /* */ comments
// The second /* inside a comment is treated as regular text
// Expected counts:
// Blank: 1
// Comment: 3 (lines 1, 2, 5)
// Code: 4 (lines 4, 6, 7, 8)
// Total: 8
