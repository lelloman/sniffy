# Sniffy

A fast command-line utility for analyzing source code statistics.

## Overview

Sniffy scans directories and generates detailed statistics about lines of code, categorized by programming language. It accurately counts blank lines, comment lines, and actual code lines.

## Features

### Currently Implemented (MVP)
- Fast and efficient processing of large codebases
- Supports 20+ programming languages (JavaScript, TypeScript, Rust, Python, Go, Java, C/C++, C#, Ruby, Shell, HTML, CSS, SCSS, Sass, Markdown, JSON, YAML, XML, PHP)
- Respects `.gitignore` patterns automatically
- Accurate line classification (blank, comment, code)
- Multi-line comment support
- Hidden file filtering
- Clean, colored terminal table output with thousand separators
- Verbose mode for progress tracking

### Planned Features
- Git history analysis with trend tracking
- Per-day/per-week statistics
- Author-based filtering
- JSON/CSV output formats
- Custom language definitions
- Generated file detection and filtering

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at target/release/sniffy
```

## Usage

```bash
# Analyze current directory
./target/release/sniffy

# Analyze specific directory
./target/release/sniffy /path/to/project

# Include hidden files
./target/release/sniffy --hidden

# Verbose output
./target/release/sniffy --verbose

# Analyze multiple paths
./target/release/sniffy src/ tests/
```

## Example Output

```
┌──────────┬───────┬───────┬─────────┬───────┬───────┐
│ Language ┆ Files ┆ Blank ┆ Comment ┆ Code  ┆ Total │
╞══════════╪═══════╪═══════╪═════════╪═══════╪═══════╡
│ Markdown ┆ 3     ┆ 185   ┆ 0       ┆ 945   ┆ 1,130 │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ Rust     ┆ 10    ┆ 236   ┆ 219     ┆ 1,296 ┆ 1,751 │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ Total    ┆ 13    ┆ 421   ┆ 219     ┆ 2,241 ┆ 2,881 │
└──────────┴───────┴───────┴─────────┴───────┴───────┘
```

## Documentation

See [DESIGN.md](DESIGN.md) for detailed architecture and design decisions.
See [PLAN.md](PLAN.md) for the complete implementation plan.

## License

TBD
