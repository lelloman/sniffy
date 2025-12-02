# Sniffy - Design Document

## Overview

Sniffy is a command-line utility that analyzes source code directories and generates statistics about lines of code, categorized by programming language. It counts blank lines, comment lines, and actual code lines for each file type.

## Goals

- **Fast**: Efficiently process large codebases
- **Accurate**: Correctly classify lines as code, comments, or blank
- **Comprehensive**: Support many common programming languages
- **User-friendly**: Clear, formatted output in the terminal
- **Smart**: Respect `.gitignore` patterns and skip non-source files
- **Git-aware**: Analyze repository history to show code evolution over time
- **Trend analysis**: Track per-day/per-week addition/deletion statistics
- **Historical insights**: Visualize how the codebase has grown or changed

## Architecture

### Component Diagram

```
┌──────────────────────────────────────────────────────────┐
│                      CLI Entry Point                      │
│  - Parse command-line arguments                           │
│  - Validate paths                                         │
│  - Orchestrate the analysis flow                          │
│  - Determine mode: snapshot vs. history analysis          │
└────────────────────┬─────────────────────────────────────┘
                     │
          ┌──────────┴──────────┐
          ↓                     ↓
  ┌───────────────┐     ┌──────────────────┐
  │   Directory   │     │  Git History     │
  │   Walker      │     │  Analyzer        │
  │               │     │  - Read commits  │
  │               │     │  - Track changes │
  │               │     │  - Daily stats   │
  └───────┬───────┘     └────────┬─────────┘
          │                      │
          ↓                      ↓
┌──────────────────────────────────────────────────────────┐
│                   Language Detector                       │
│  - Map file extensions to languages                       │
│  - Provide language-specific parsing rules                │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────┐
│                    File Processor                         │
│  - Read file contents                                     │
│  - Detect file encoding                                   │
│  - Split into lines                                       │
│  - Delegate line classification                           │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────┐
│                   Line Classifier                         │
│  - Detect blank lines                                     │
│  - Detect comment lines (single & multi-line)             │
│  - Track multi-line comment state                         │
│  - Handle mixed lines (code + comments)                   │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────┐
│                 Statistics Aggregator                     │
│  - Accumulate counts per language                         │
│  - Track file counts                                      │
│  - Calculate totals across all languages                  │
│  - Build time-series data for history mode                │
└────────────────────┬─────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────┐
│                    Output Formatter                       │
│  - Format statistics as terminal table                    │
│  - Sort by language or line count                         │
│  - Apply colors/styling                                   │
│  - Generate trend charts for history mode                 │
└──────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Language Definition System

Each programming language is defined by:

```rust
struct LanguageInfo {
    name: &'static str,
    extensions: Vec<&'static str>,
    single_line_comments: Vec<&'static str>,
    multi_line_comments: Vec<CommentPair>,
}

struct CommentPair {
    start: &'static str,
    end: &'static str,
}
```

**Initial Language Support:**
- JavaScript/TypeScript (`.js`, `.jsx`, `.ts`, `.tsx`)
- Python (`.py`)
- Rust (`.rs`)
- Go (`.go`)
- Java (`.java`)
- C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)
- C# (`.cs`)
- Ruby (`.rb`)
- Shell (`.sh`, `.bash`)
- HTML (`.html`, `.htm`)
- CSS/SCSS (`.css`, `.scss`, `.sass`)
- Markdown (`.md`)
- JSON (`.json`)
- YAML (`.yaml`, `.yml`)
- XML (`.xml`)

### 2. Line Classification Algorithm

Lines are classified in order of precedence:

1. **Blank Line**: Contains only whitespace
2. **Comment Line**: Starts with comment syntax (after trimming whitespace)
3. **Code Line**: Everything else

**State Machine for Multi-line Comments:**
```
State: NotInComment | InComment

For each line:
  1. Trim leading/trailing whitespace
  2. If empty → Blank
  3. If InComment:
     - If contains end delimiter → check if code after it
     - Otherwise → Comment
  4. If NotInComment:
     - If starts with multi-line start → check if ends on same line
     - If starts with single-line comment → Comment
     - Otherwise → Code
```

**Edge Cases:**

- **Mixed lines** (code + comment): Count as code
  ```rust
  let x = 5; // comment
  ```
  → Code line

- **Multi-line comments with code after**:
  ```rust
  /* comment */ let x = 5;
  ```
  → Code line

- **Strings containing comment syntax**:
  ```rust
  let s = "// not a comment";
  ```
  → Code line (v1: simple heuristic, may misclassify)

### 3. File Walking Strategy

**Use `ignore` crate** (same library used by ripgrep):
- Automatically respects `.gitignore`
- Handles `.ignore` files
- Fast and battle-tested

**Skip patterns:**
- Binary files (detect by checking for null bytes in first 8KB)
- Hidden files/directories (`.git`, `.svn`, etc.)
- Common build/dependency directories: `node_modules`, `target`, `build`, `dist`, `.venv`
- Lock files, minified files (`.min.js`, `.min.css`)

**Parallel processing:**
- Use Rayon for parallel file processing (when beneficial)
- Single-threaded for small projects (<100 files)

### 4. Statistics Structure

```rust
#[derive(Default)]
struct FileStats {
    blank: usize,
    comment: usize,
    code: usize,
}

impl FileStats {
    fn total(&self) -> usize {
        self.blank + self.comment + self.code
    }
}

struct LanguageStats {
    language: String,
    files: usize,
    stats: FileStats,
}

struct ProjectStats {
    languages: HashMap<String, LanguageStats>,
}

// For history analysis
struct DailyStats {
    date: NaiveDate,
    additions: FileStats,
    deletions: FileStats,
    net_change: i64,  // Can be negative
}

struct HistoricalStats {
    daily: Vec<DailyStats>,
    by_author: HashMap<String, ProjectStats>,
    total_commits: usize,
}
```

### 5. Git History Analyzer

**Purpose:** Analyze git commit history to track code evolution over time.

**Key operations:**
- Walk through git commits in chronological order
- For each commit, analyze the diff to determine additions/deletions
- Classify added/deleted lines as code, comment, or blank
- Aggregate by date, language, and optionally by author
- Handle file renames and moves

**Implementation notes:**
- Use `git2-rs` or `gix` crate for git operations
- Process commits in batches for large repositories
- Cache results to avoid re-analysis
- Handle edge cases: merge commits, binary files, large diffs

**Output examples:**
```
Daily additions (last 7 days):

Date         Added    Deleted    Net      Languages
──────────────────────────────────────────────────────
2024-01-15    +234      -45     +189     Rust, Python
2024-01-16    +567      -123    +444     JavaScript
2024-01-17    +89       -234    -145     Python
...

Top contributors (by lines added):
1. Alice    (+2,345 lines)
2. Bob      (+1,234 lines)
3. Charlie  (+987 lines)
```

## CLI Interface

### Basic Usage

```bash
# Analyze current directory
sniffy

# Analyze specific directory
sniffy /path/to/project

# Analyze multiple paths
sniffy src/ tests/
```

### Options

```bash
# Exclude patterns
sniffy --exclude "*.min.js" --exclude "vendor/"

# Include hidden files
sniffy --hidden

# Show only specific languages
sniffy --languages rust,python

# Sort output
sniffy --sort-by lines  # or: files, language (default)

# Verbose mode (show progress)
sniffy --verbose

# Git history analysis
sniffy --history                    # Show evolution over entire history
sniffy --since "2024-01-01"         # Analyze changes since date
sniffy --last 30                    # Last N days
sniffy --by-day                     # Per-day statistics
sniffy --by-week                    # Per-week statistics
sniffy --author "name"              # Filter by author
```

### Output Format

```
Analyzing: /home/user/project

Language       Files      Blank    Comment       Code      Total
────────────────────────────────────────────────────────────────
Rust              45        834      1,247      8,456      10,537
JavaScript        32        567        892      5,234       6,693
Python            18        234        445      2,891       3,570
TypeScript        12        189        321      1,876       2,386
Markdown           8         45          0        234         279
JSON               5          0          0        127         127
YAML               3         12          5         89         106
────────────────────────────────────────────────────────────────
Total            123      1,881      2,910     18,907      23,698
```

## Implementation Phases

### Phase 1: Core Functionality
- Basic line counting (blank, code, comments)
- Support for 5-10 common languages
- Simple directory walking
- Terminal table output

### Phase 2: Robustness
- Proper .gitignore support
- Handle encoding issues
- Better error messages
- Progress indication for large projects
- String literal detection (v2 feature - proper parsing to avoid false positives)

### Phase 3: Git Integration
- Read git commit history
- Analyze per-commit changes (additions/deletions)
- Per-day and per-week aggregation
- Author filtering
- Time-series visualization in terminal

### Phase 4: Polish
- Parallel processing optimization
- Comprehensive language support (20+ languages)
- Colorized output
- Additional sorting options
- Generated file detection

## Dependencies

- **clap** (v4): CLI argument parsing with derive macros
- **ignore**: Directory walking with .gitignore support
- **comfy-table**: Terminal table formatting
- **anyhow**: Error handling
- **rayon**: (Optional) Parallel processing
- **git2** or **gix**: Git repository interaction for history analysis

## Error Handling

**Graceful degradation:**
- If a file can't be read → skip it, log warning
- If a file has encoding issues → skip it, log warning
- If a directory can't be accessed → skip it, log warning
- Unknown file extensions → skip silently (unless verbose)

**Exit codes:**
- 0: Success
- 1: Invalid arguments
- 2: Path not found
- 3: Permission denied

## Performance Considerations

**Target performance:**
- ~1000 files/second on modern hardware
- Minimal memory footprint (<50MB for typical projects)

**Optimizations:**
- Read files in chunks, not entire contents at once
- Use `BufReader` for efficient line-by-line reading
- Avoid unnecessary string allocations
- Use parallel processing for large projects (>500 files)

## Testing Strategy

**Unit tests:**
- Line classifier with various edge cases
- Language detector
- Comment state machine

**Integration tests:**
- Test against real codebases
- Fixtures with known line counts

**Test fixtures:**
```
tests/
  fixtures/
    simple/          (small project, hand-counted)
    edge_cases/      (tricky comment scenarios)
    multi_lang/      (mixed language project)
```

## Future Enhancements (v2+)

- JSON/CSV output formats
- **String literal detection**: Proper language-specific parsing to avoid false positives when comment syntax appears inside strings (Phase 2 feature)
- Configuration file support (`.sniffyrc`)
- Exclude/include patterns in config
- Custom language definitions
- Web dashboard for visualizing stats over time

## Design Decisions

### Line Classification Rules

1. **Mixed lines** (code + comment on same line): Count as **code**
   - Example: `let x = 5; // comment` → Code line

2. **Shebang lines**: Count as **code**
   - Example: `#!/usr/bin/env python` → Code line

3. **Documentation comments**: Count as regular **comments**
   - JSDoc, Rustdoc, etc. are treated like any other comment

4. **Comment-only lines**: Count as **comment** only if the line contains nothing but comment syntax and text
   - Example: `// this is a comment` → Comment line
   - Example: `x = 5; // comment` → Code line

### Generated File Detection

Detect auto-generated files to optionally skip them in analysis:

**Detection strategies:**
1. **Comment header patterns** (check first 10 lines):
   - `@generated`
   - `AUTO-GENERATED`
   - `AUTOGENERATED`
   - `DO NOT EDIT`
   - `Code generated by`
   - `Generated by`
   - `<auto-generated`

2. **File extension patterns**:
   - `*.pb.go` (Protocol Buffer - Go)
   - `*.pb.h`, `*.pb.cc` (Protocol Buffer - C++)
   - `*.g.cs` (Generated C#)
   - `*.designer.cs` (Visual Studio designer)
   - `*.g.dart` (Generated Dart)
   - `*.gen.ts`, `*.gen.js` (Generic generated files)

3. **Directory patterns**:
   - `**/generated/**`
   - `**/gen/**`
   - `**/__generated__/**`

**CLI option:** `--skip-generated` (default: include generated files)

## References

- Existing tools: cloc, tokei, scc
- [ripgrep's ignore crate](https://docs.rs/ignore/)
- [comfy-table docs](https://docs.rs/comfy-table/)
