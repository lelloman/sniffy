# 🐕 Sniffy

> A blazingly fast source code lines counter, written in Rust, in fact is blazingly fast.

Sniffy is a command-line tool for analyzing source code lines statistics across multiple programming languages. It provides accurate line counting, git history analysis, and multiple output formats with high performance through parallel processing.

## ✨ Features

- **⚡ Blazingly Fast** - Parallel processing across all CPU cores for maximum performance FOR FREE.
- **🎯 Accurate Line Classification** - Distinguishes code, comments, and blank lines with multi-line comment support
- **📊 Git History Analysis** - Track code changes over time with daily/weekly aggregation and contributor stats
- **🌈 Multiple Output Formats** - Beautiful tables, JSON, or CSV output
- **🎨 33+ Languages Supported** - From JavaScript to Rust, Python to Haskell
- **🚫 Smart Filtering** - Respects `.gitignore` patterns automatically
- **🔧 Highly Configurable** - Control parallelism, output format, filtering, and more
- **👷 Totally written by a human being** - Made for humans, by humans

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/lelloman/sniffy.git
cd sniffy

# Build with optimizations
cargo build --release

# Binary will be at target/release/sniffy
```

### Using Cargo (when published)

```bash
cargo install sniffy
```

## 🚀 Quick Start

```bash
# Analyze current directory
sniffy

# Analyze specific directory
sniffy /path/to/project

# Multiple paths
sniffy src/ tests/ examples/

# With verbose output
sniffy --verbose
```

## 📖 Usage

### Basic Analysis

```bash
# Current directory with default settings
sniffy .

# Include hidden files
sniffy --hidden

# Control parallel jobs (0 = auto-detect CPUs)
sniffy --jobs 4

# Disable colored output
sniffy --no-color

# Or use the NO_COLOR environment variable (https://no-color.org/)
NO_COLOR=1 sniffy
```

### Output Formats

#### Table Format (Default)

```bash
sniffy /path/to/large/project
```

Output (65,002 files, 35M lines analyzed):

```
┌────────────┬────────┬───────────┬───────────┬────────────┬────────────┐
│ Language   ┆ Files  ┆ Blank     ┆ Comment   ┆ Code       ┆ Total      │
╞════════════╪════════╪═══════════╪═══════════╪════════════╪════════════╡
│ C          ┆ 58,810 ┆ 4,210,565 ┆ 4,148,311 ┆ 25,426,269 ┆ 33,785,145 │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Shell      ┆ 880    ┆ 30,898    ┆ 20,992    ┆ 120,570    ┆ 172,460    │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ JSON       ┆ 816    ┆ 2         ┆ 0         ┆ 460,461    ┆ 460,463    │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ YAML       ┆ 4,105  ┆ 75,795    ┆ 18,962    ┆ 366,183    ┆ 460,940    │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Python     ┆ 228    ┆ 10,797    ┆ 8,653     ┆ 52,831     ┆ 72,281     │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ ...14 more │        │           │           │            │            │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Total      ┆ 65,002 ┆ 4,337,126 ┆ 4,209,297 ┆ 26,484,680 ┆ 35,031,103 │
└────────────┴────────┴───────────┴───────────┴────────────┴────────────┘
```

#### JSON Format

```bash
sniffy src/ --format json
```

Output:

```json
{
  "languages": [
    {
      "language": "Rust",
      "files": 11,
      "stats": {
        "blank": 349,
        "comment": 337,
        "code": 1892
      }
    }
  ],
  "total_files": 15,
  "total_stats": {
    "blank": 547,
    "comment": 337,
    "code": 2900
  }
}
```

#### CSV Format

```bash
sniffy src/ --format csv
```

Output:

```csv
language,files,blank,comment,code,total
Markdown,3,195,0,983,1178
Rust,11,349,337,1892,2578
TOML,1,3,0,25,28
Total,15,547,337,2900,3784
```

### Git History Analysis

Analyze your repository's evolution over time:

```bash
# Basic history analysis
sniffy --history

# Since a specific date
sniffy --history --since 2024-01-01

# Date range (from/to)
sniffy --history --since 2024-01-01 --until 2024-12-31

# Last N days (convenient shorthand)
sniffy --history --last 30

# Weekly aggregation
sniffy --history --by-week

# Filter by author
sniffy --history --author "John Doe"

# Verbose mode (shows progress for large repos)
sniffy --history --verbose

# Combine filters with JSON output
sniffy --history --since 2024-01-01 --by-week --format json
```

History output example:

```
Git History Analysis
Total Commits: 18
Date Range: 2025-12-01 to 2025-12-02

Daily Statistics:
┌────────────┬───────┬─────────┬────────────┐
│ Date       ┆ Added ┆ Deleted ┆ Net Change │
╞════════════╪═══════╪═════════╪════════════╡
│ 2025-12-02 ┆ 3,981 ┆ 247     ┆ +3,734     │
└────────────┴───────┴─────────┴────────────┘

Top Contributors:
┌────────────────────┬────────────┬──────────┬───────┐
│ Author             ┆ Code Lines ┆ Comments ┆ Total │
╞════════════════════╪════════════╪══════════╪═══════╡
│ Domenico Cerasuolo ┆ 3,981      ┆ 565      ┆ 5,211 │
└────────────────────┴────────────┴──────────┴───────┘
```

## 🎯 Command-Line Options

```
Usage: sniffy [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Paths to analyze (defaults to current directory)

Options:
  -H, --hidden           Include hidden files and directories
  -v, --verbose          Verbose output with progress tracking
  -j, --jobs <N>         Number of parallel jobs (0 = number of CPUs) [default: 0]
      --format <FORMAT>  Output format (table, json, or csv) [default: table]
      --no-color         Disable colored output

  Git History Options:
      --history          Analyze git commit history
      --since <DATE>     Only analyze commits since date (YYYY-MM-DD or RFC3339)
      --until <DATE>     Only analyze commits until date (YYYY-MM-DD or RFC3339)
      --last <N>         Only analyze commits from the last N days
      --by-day           Group history by day (default)
      --by-week          Group history by week
      --author <NAME>    Filter commits by author name

  -h, --help             Print help
  -V, --version          Print version
```

## 🌍 Supported Languages

Sniffy supports 33+ programming languages including:

| Language   | Extensions                  |
| ---------- | --------------------------- |
| JavaScript | .js, .jsx, .mjs, .cjs       |
| TypeScript | .ts, .tsx                   |
| Rust       | .rs                         |
| Python     | .py                         |
| Go         | .go                         |
| Java       | .java                       |
| C          | .c, .h                      |
| C++        | .cpp, .cc, .cxx, .hpp, .hxx |
| C#         | .cs                         |
| Ruby       | .rb                         |
| PHP        | .php                        |
| Shell      | .sh, .bash, .zsh            |
| Kotlin     | .kt, .kts                   |
| Swift      | .swift                      |
| Scala      | .scala                      |
| Haskell    | .hs                         |
| Elixir     | .ex, .exs                   |
| Erlang     | .erl                        |
| Lua        | .lua                        |
| Perl       | .pl, .pm                    |
| R          | .r, .R                      |
| SQL        | .sql                        |
| HTML       | .html, .htm                 |
| CSS        | .css                        |
| SCSS/Sass  | .scss, .sass                |
| Markdown   | .md, .markdown              |
| JSON       | .json                       |
| YAML       | .yaml, .yml                 |
| XML        | .xml                        |
| TOML       | .toml                       |
| INI        | .ini, .cfg                  |
| Vim Script | .vim                        |

## 📊 Use Cases

- **Project Health Monitoring** - Track codebase growth and evolution
- **Code Review Metrics** - Understand contribution patterns
- **Documentation Ratios** - Measure comment-to-code ratios
- **Language Distribution** - See what languages comprise your project
- **CI/CD Metrics** - Integrate into build pipelines for statistics
- **Historical Analysis** - Understand how your project evolved over time

## 🛠️ Development

### Building from Source

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run with specific verbosity
cargo run -- /path/to/analyze --verbose
```

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_classify_line
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings
```

## 📝 Documentation

- That's where you're at.

## 🤝 Contributing

Contributions are welcome! Areas where help is appreciated:

- Additional language support
- Performance optimizations
- Bug fixes and edge cases
- Documentation improvements
- Feature requests and ideas

## 📄 License

MIT OR Apache-2.0

---

**Made with ❤️ and 🦀 Rust** and humans
