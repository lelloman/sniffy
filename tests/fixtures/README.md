# Test Fixtures

This directory contains test fixtures with known line counts for integration testing.

## Simple Fixtures (`simple/`)

Small files with straightforward, easily verified line counts.

### `main.rs`
- **Blank**: 2
- **Comment**: 4
- **Code**: 6
- **Total**: 12

### `script.py`
- **Blank**: 3
- **Comment**: 6 (including docstring lines)
- **Code**: 6
- **Total**: 15

## Edge Cases (`edge_cases/`)

Files testing tricky scenarios like multi-line comments, mixed lines, etc.

### `comments.js`
- **Blank**: 2
- **Comment**: 7
- **Code**: 5
- **Total**: 14

Note: Lines with both code and comments count as code.

### `only_comments.rs`
- **Blank**: 0
- **Comment**: 5
- **Code**: 0
- **Total**: 5

### `only_code.rs`
- **Blank**: 0
- **Comment**: 0
- **Code**: 5
- **Total**: 5

### `only_blank.txt`
- **Blank**: 5
- **Comment**: 0
- **Code**: 0
- **Total**: 5

Note: This file has no extension, so it won't be processed by sniffy.

### `long_multiline.rs`
- **Blank**: 2
- **Comment**: 13
- **Code**: 3
- **Total**: 18

### `mixed.rs`
- **Blank**: 2
- **Comment**: 2
- **Code**: 5
- **Total**: 9

### `binary.bin`
Should be skipped as binary file.

### `generated.go`
Contains `@generated` header - could be used for testing generated file detection.

## Multi-Language (`multi_lang/`)

Project with files in different languages.

### `app.js`
- **Blank**: 0
- **Comment**: 1
- **Code**: 3
- **Total**: 4

### `lib.rs`
- **Blank**: 0
- **Comment**: 1
- **Code**: 3
- **Total**: 4

### `util.py`
- **Blank**: 0
- **Comment**: 1
- **Code**: 2
- **Total**: 3

### Multi-lang Total
- **Files**: 3
- **Blank**: 0
- **Comment**: 3
- **Code**: 8
- **Total**: 11
