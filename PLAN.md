# Sniffy - Complete Implementation Plan

This document contains every single task in the exact sequence they should be implemented.

---

## Phase 1: Project Setup and Foundation

### 1.1 Initialize Rust Project
- [x] Run `cargo init --name sniffy`
- [x] Create basic directory structure: `src/`, `tests/`, `benches/`
- [x] Add `.gitignore` for Rust projects (target/, Cargo.lock for binaries)
- [x] Create README.md with basic project description

### 1.2 Configure Cargo.toml
- [x] Add project metadata (name, version, authors, edition, description)
- [x] Add dependency: `clap = { version = "4", features = ["derive"] }`
- [x] Add dependency: `anyhow = "1"`
- [x] Add dependency: `comfy-table = "7"`
- [x] Add dependency: `ignore = "0.4"`
- [x] Add dev-dependency: `tempfile = "3"` (for tests)
- [x] Configure binary target in Cargo.toml

### 1.3 Create Project Structure
- [x] Create `src/main.rs` (entry point)
- [x] Create `src/lib.rs` (library root)
- [x] Create `src/language.rs` (language definitions)
- [x] Create `src/classifier.rs` (line classification)
- [x] Create `src/processor.rs` (file processing)
- [x] Create `src/walker.rs` (directory walking)
- [x] Create `src/stats.rs` (statistics aggregation)
- [x] Create `src/output.rs` (output formatting)
- [x] Create `src/cli.rs` (CLI argument parsing)
- [x] Create `src/error.rs` (custom error types)

---

## Phase 2: Core Data Structures

### 2.1 Statistics Data Structures (src/stats.rs)
- [x] Define `FileStats` struct with fields: blank, comment, code
- [x] Implement `Default` trait for `FileStats`
- [x] Implement `total()` method for `FileStats`
- [x] Implement `Add` trait for `FileStats` to allow combining stats
- [x] Implement `AddAssign` trait for `FileStats`
- [x] Define `LanguageStats` struct with fields: language, files, stats
- [x] Implement `Default` trait for `LanguageStats`
- [x] Define `ProjectStats` struct with HashMap<String, LanguageStats>
- [x] Implement `new()` constructor for `ProjectStats`
- [x] Implement `add_file_stats()` method to add stats for a language
- [x] Implement `get_languages()` method to return sorted list of languages
- [x] Implement `total()` method to calculate totals across all languages
- [x] Add unit tests for stats combining and totaling

### 2.2 Error Types (src/error.rs)
- [x] Define custom error enum `SniffyError`
- [x] Add variant for IO errors
- [x] Add variant for invalid path errors
- [x] Add variant for encoding errors
- [x] Add variant for file processing errors
- [x] Implement `Display` trait for `SniffyError`
- [x] Implement `From<std::io::Error>` for `SniffyError`
- [x] Create type alias `Result<T> = std::result::Result<T, SniffyError>`

---

## Phase 3: Language System

### 3.1 Language Definition Structures (src/language.rs)
- [ ] Define `CommentPair` struct with start and end delimiters
- [ ] Define `LanguageInfo` struct with name, extensions, single_line_comments, multi_line_comments
- [ ] Implement constructor method for `LanguageInfo`

### 3.2 Language Definitions (src/language.rs)
- [ ] Create `LANGUAGES` static array containing all language definitions
- [ ] Add JavaScript language definition (.js, .jsx)
- [ ] Add TypeScript language definition (.ts, .tsx)
- [ ] Add Python language definition (.py)
- [ ] Add Rust language definition (.rs)
- [ ] Add Go language definition (.go)
- [ ] Add Java language definition (.java)
- [ ] Add C language definition (.c, .h)
- [ ] Add C++ language definition (.cpp, .cc, .cxx, .hpp, .hxx)
- [ ] Add C# language definition (.cs)
- [ ] Add Ruby language definition (.rb)
- [ ] Add Shell language definition (.sh, .bash, .zsh)
- [ ] Add HTML language definition (.html, .htm)
- [ ] Add CSS language definition (.css)
- [ ] Add SCSS/Sass language definition (.scss, .sass)
- [ ] Add Markdown language definition (.md, .markdown)
- [ ] Add JSON language definition (.json)
- [ ] Add YAML language definition (.yaml, .yml)
- [ ] Add XML language definition (.xml)
- [ ] Add PHP language definition (.php)

### 3.3 Language Detector (src/language.rs)
- [ ] Create `LanguageDetector` struct
- [ ] Implement `new()` that builds extension-to-language HashMap
- [ ] Implement `detect_from_path()` method that takes a Path and returns Option<&LanguageInfo>
- [ ] Handle case-insensitive extension matching
- [ ] Add unit tests for language detection by extension
- [ ] Test unknown extensions return None
- [ ] Test case insensitivity (.RS vs .rs)

---

## Phase 4: Line Classification Engine

### 4.1 Line Classifier State (src/classifier.rs)
- [ ] Define `LineType` enum: Blank, Comment, Code
- [ ] Define `ClassifierState` struct to track multi-line comment state
- [ ] Add field `in_multi_line_comment` to track if we're inside a multi-line comment
- [ ] Add field `current_delimiter` to track which delimiter we're inside
- [ ] Implement `new()` constructor for `ClassifierState`
- [ ] Implement `reset()` method to reset state

### 4.2 Line Classification Logic (src/classifier.rs)
- [ ] Create `LineClassifier` struct
- [ ] Implement `new()` constructor that takes a reference to `LanguageInfo`
- [ ] Create helper method `trim_line()` to trim whitespace
- [ ] Create helper method `is_blank()` to check if line is blank
- [ ] Create helper method `starts_with_single_comment()` to check single-line comments
- [ ] Create helper method `contains_multi_line_start()` to find multi-line start delimiters
- [ ] Create helper method `contains_multi_line_end()` to find multi-line end delimiters
- [ ] Create helper method `find_delimiter_position()` to locate delimiter in line
- [ ] Implement main `classify_line()` method that takes a line and state, returns LineType
- [ ] Handle blank line detection (first check)
- [ ] Handle case: already in multi-line comment
  - [ ] Check if line contains end delimiter
  - [ ] If end found, check if there's code after the delimiter
  - [ ] If code after delimiter, return Code and update state
  - [ ] If no code after delimiter, return Comment and update state
  - [ ] If no end delimiter, return Comment (stay in comment)
- [ ] Handle case: not in multi-line comment
  - [ ] Check if line starts with single-line comment → return Comment
  - [ ] Check if line starts with multi-line comment start
  - [ ] If multi-line start found, check if end is on same line
  - [ ] If complete multi-line on one line, check for code after → return Code or Comment
  - [ ] If multi-line starts but doesn't end, update state and return Comment
  - [ ] Otherwise return Code
- [ ] Handle shebang lines (#!) as Code
- [ ] Add comprehensive unit tests for line classification:
  - [ ] Test blank lines
  - [ ] Test single-line comments
  - [ ] Test code lines
  - [ ] Test multi-line comments (start, middle, end)
  - [ ] Test mixed lines (code + comment)
  - [ ] Test nested multi-line comments
  - [ ] Test edge case: empty multi-line comment /* */
  - [ ] Test shebang lines
  - [ ] Test strings containing comment syntax (will misclassify in v1, document)

### 4.3 File Classification (src/classifier.rs)
- [ ] Create `classify_file()` function that takes lines and language info
- [ ] Initialize `ClassifierState`
- [ ] Initialize `FileStats`
- [ ] Iterate through all lines
- [ ] Classify each line and update stats
- [ ] Return final `FileStats`
- [ ] Add unit tests with sample file content

---

## Phase 5: File Processing

### 5.1 Binary File Detection (src/processor.rs)
- [ ] Create `is_binary_file()` function
- [ ] Read first 8KB of file
- [ ] Check for null bytes (0x00)
- [ ] Return true if null bytes found (likely binary)
- [ ] Handle IO errors gracefully
- [ ] Add unit tests with text and binary test files

### 5.2 File Processor (src/processor.rs)
- [ ] Create `FileProcessor` struct
- [ ] Add field for `LanguageDetector`
- [ ] Implement `new()` constructor
- [ ] Implement `process_file()` method that takes a Path
  - [ ] Check if file is binary, skip if true
  - [ ] Detect language from file extension
  - [ ] If language unknown, return None
  - [ ] Open file with BufReader
  - [ ] Read file line by line
  - [ ] Handle UTF-8 decoding errors (skip invalid lines with warning)
  - [ ] Collect all lines into Vec<String>
  - [ ] Call line classifier to get FileStats
  - [ ] Return Some((language_name, FileStats))
- [ ] Handle IO errors with proper error types
- [ ] Add unit tests with sample files

---

## Phase 6: Directory Walking

### 6.1 Basic Walker (src/walker.rs)
- [ ] Create `DirectoryWalker` struct
- [ ] Add field `builder` for ignore::WalkBuilder
- [ ] Add field `hidden` flag for including hidden files
- [ ] Implement `new()` constructor that takes a path
- [ ] Configure WalkBuilder to respect .gitignore by default
- [ ] Implement `hidden()` method to configure hidden file inclusion
- [ ] Implement `walk()` method that returns an iterator of paths
- [ ] Filter out directories (only return files)
- [ ] Handle errors during walking (permission denied, etc.)

### 6.2 File Filtering (src/walker.rs)
- [ ] Add common skip patterns to walker:
  - [ ] `node_modules/`
  - [ ] `target/` (Rust)
  - [ ] `.git/`
  - [ ] `.svn/`
  - [ ] `build/`
  - [ ] `dist/`
  - [ ] `.venv/` and `venv/`
  - [ ] `__pycache__/`
  - [ ] `*.min.js`
  - [ ] `*.min.css`
  - [ ] Lock files (package-lock.json, Cargo.lock, etc.)
- [ ] Create `should_skip_file()` helper function
- [ ] Add unit tests for skip patterns

### 6.3 Generated File Detection (src/walker.rs)
- [ ] Create `GeneratedFileDetector` struct
- [ ] Define header patterns to check: @generated, AUTO-GENERATED, DO NOT EDIT, etc.
- [ ] Define file extension patterns: *.pb.go, *.g.cs, *.gen.ts, etc.
- [ ] Define directory patterns: **/generated/**, **/gen/**, **/__generated__/**
- [ ] Implement `is_generated()` method that checks all patterns
  - [ ] Check file extension first (fast check)
  - [ ] Check directory path patterns
  - [ ] Read first 10 lines and check for header patterns
- [ ] Add configuration flag to enable/disable generated file skipping
- [ ] Add unit tests with generated file examples

---

## Phase 7: CLI Interface

### 7.1 CLI Structure (src/cli.rs)
- [ ] Create `Cli` struct with clap derive macro
- [ ] Add field `paths`: Vec<PathBuf> (default to current directory)
- [ ] Add field `exclude`: Vec<String> for exclude patterns
- [ ] Add field `hidden`: bool for including hidden files
- [ ] Add field `languages`: Option<Vec<String>> to filter languages
- [ ] Add field `sort_by`: enum (Language, Files, Lines) with default
- [ ] Add field `verbose`: bool for verbose output
- [ ] Add field `skip_generated`: bool (default false)
- [ ] Add subcommand structure for future `history` command
- [ ] Implement `parse()` method to parse arguments

### 7.2 CLI Validation (src/cli.rs)
- [ ] Implement `validate()` method to check arguments
- [ ] Validate that paths exist
- [ ] Validate that paths are readable
- [ ] Convert relative paths to absolute paths
- [ ] Return errors for invalid arguments
- [ ] Add unit tests for validation

---

## Phase 8: Output Formatting

### 8.1 Table Formatter (src/output.rs)
- [ ] Create `OutputFormatter` struct
- [ ] Implement `format_table()` method that takes ProjectStats
- [ ] Create comfy-table Table instance
- [ ] Set table header: Language, Files, Blank, Comment, Code, Total
- [ ] Add rows for each language with stats
- [ ] Calculate and add Total row at bottom
- [ ] Format numbers with thousand separators
- [ ] Right-align numeric columns
- [ ] Apply borders and styling
- [ ] Return formatted string

### 8.2 Sorting (src/output.rs)
- [ ] Implement `sort_languages()` method that takes sort criteria
- [ ] Sort by language name (alphabetical)
- [ ] Sort by file count (descending)
- [ ] Sort by line count (descending)
- [ ] Add unit tests for sorting

### 8.3 Progress Indicator (src/output.rs)
- [ ] Create `ProgressIndicator` struct
- [ ] Track files processed counter
- [ ] Implement `update()` method to show progress
- [ ] Show current file being processed in verbose mode
- [ ] Show total files processed every N files
- [ ] Implement `finish()` method to show completion message

---

## Phase 9: Main Integration

### 9.1 Main Application Flow (src/main.rs)
- [ ] Parse CLI arguments
- [ ] Validate arguments and handle errors
- [ ] Create LanguageDetector instance
- [ ] Create FileProcessor instance
- [ ] Create ProjectStats instance
- [ ] Create ProgressIndicator if verbose mode enabled
- [ ] For each path in arguments:
  - [ ] Create DirectoryWalker
  - [ ] Configure walker with hidden/exclude options
  - [ ] Walk directory and collect file paths
- [ ] Process each file:
  - [ ] Call FileProcessor.process_file()
  - [ ] Add results to ProjectStats
  - [ ] Update progress indicator
  - [ ] Handle errors gracefully (log warning, continue)
- [ ] Create OutputFormatter
- [ ] Sort results based on sort criteria
- [ ] Format and print table
- [ ] Handle errors at top level and exit with appropriate code
- [ ] Add integration test for basic run

### 9.2 Error Handling (src/main.rs)
- [ ] Wrap main logic in Result
- [ ] Map errors to exit codes (1, 2, 3)
- [ ] Print user-friendly error messages
- [ ] Handle Ctrl+C gracefully
- [ ] Clean up resources on error

---

## Phase 10: Testing

### 10.1 Test Infrastructure
- [ ] Create `tests/` directory for integration tests
- [ ] Create `tests/fixtures/` for test files
- [ ] Create `tests/fixtures/simple/` - small project with known line counts
- [ ] Create `tests/fixtures/edge_cases/` - tricky comment scenarios
- [ ] Create `tests/fixtures/multi_lang/` - mixed language project

### 10.2 Test Fixtures
- [ ] Create fixture: simple Rust file with known counts
- [ ] Create fixture: simple Python file with known counts
- [ ] Create fixture: JavaScript file with various comment types
- [ ] Create fixture: file with only comments
- [ ] Create fixture: file with only code
- [ ] Create fixture: file with only blank lines
- [ ] Create fixture: file with multi-line comments spanning many lines
- [ ] Create fixture: file with nested comments (if language supports)
- [ ] Create fixture: file with mixed code and comments
- [ ] Create fixture: binary file for binary detection test
- [ ] Create fixture: generated file with @generated header
- [ ] Document expected counts for each fixture in README

### 10.3 Unit Tests
- [ ] Test FileStats addition and combining
- [ ] Test LanguageDetector with various extensions
- [ ] Test LineClassifier with all edge cases
- [ ] Test binary file detection
- [ ] Test generated file detection
- [ ] Test skip patterns
- [ ] Test sorting algorithms
- [ ] Each test should verify exact expected behavior

### 10.4 Integration Tests
- [ ] Test: Run sniffy on simple fixture, verify output matches expected
- [ ] Test: Run with --exclude pattern, verify files are skipped
- [ ] Test: Run with --hidden, verify hidden files are included
- [ ] Test: Run with --languages filter, verify only those languages shown
- [ ] Test: Run with --skip-generated, verify generated files skipped
- [ ] Test: Run with invalid path, verify error code 2
- [ ] Test: Run with empty directory, verify graceful handling
- [ ] Test: Run with permission denied directory, verify warning and continue

### 10.5 Manual Testing
- [ ] Test on small real project (e.g., sniffy itself)
- [ ] Test on medium project (1000+ files)
- [ ] Test on large project (10000+ files)
- [ ] Compare results with other tools (tokei, cloc) for sanity check
- [ ] Test on Windows (if available)
- [ ] Test on macOS (if available)
- [ ] Test on Linux

---

## Phase 11: Git History Analysis (Phase 3 Features)

### 11.1 Git Dependencies
- [ ] Add dependency: `git2 = "0.18"` to Cargo.toml
- [ ] Add dependency: `chrono = "0.4"` for date handling

### 11.2 Git Data Structures (src/stats.rs)
- [ ] Define `DailyStats` struct with date, additions, deletions, net_change
- [ ] Define `HistoricalStats` struct with daily Vec, by_author HashMap, total_commits
- [ ] Implement methods to aggregate stats by day/week
- [ ] Implement methods to filter by date range
- [ ] Implement methods to filter by author
- [ ] Add unit tests for aggregation logic

### 11.3 Git Repository Detection (src/git.rs)
- [ ] Create `src/git.rs` module
- [ ] Create `GitAnalyzer` struct
- [ ] Implement `new()` that takes a repository path
- [ ] Implement `is_git_repo()` to check if path is in a git repository
- [ ] Implement `open_repo()` to open git2::Repository
- [ ] Handle errors when not a git repo
- [ ] Add unit tests with temp git repos

### 11.4 Commit Walking (src/git.rs)
- [ ] Implement `walk_commits()` method to iterate through commits
- [ ] Accept date range filter (since, until)
- [ ] Accept author filter
- [ ] Sort commits chronologically
- [ ] Handle merge commits (skip or include both parents)
- [ ] Implement efficient iteration using git2's revwalk
- [ ] Add unit tests with small test repository

### 11.5 Diff Analysis (src/git.rs)
- [ ] Implement `analyze_commit()` method that analyzes a single commit
- [ ] Get diff between commit and parent
- [ ] Iterate through diff hunks
- [ ] For each added line, classify as blank/comment/code
- [ ] For each deleted line, classify as blank/comment/code
- [ ] Detect language from file path
- [ ] Skip binary files
- [ ] Handle file renames and moves
- [ ] Return DailyStats for the commit
- [ ] Add unit tests with known diffs

### 11.6 Line Classification in Diffs (src/git.rs)
- [ ] Create `classify_diff_line()` helper that classifies a single line
- [ ] Reuse LineClassifier logic
- [ ] Handle partial lines (no newline at end)
- [ ] Handle context lines vs added/deleted lines
- [ ] Track multi-line comment state across diff hunks (challenging!)
- [ ] Add unit tests for diff line classification

### 11.7 History Aggregation (src/git.rs)
- [ ] Implement `analyze_history()` method that analyzes full history
- [ ] Walk all commits in date range
- [ ] Analyze each commit
- [ ] Aggregate daily stats
- [ ] Build by-author stats if requested
- [ ] Return HistoricalStats
- [ ] Show progress for large repositories
- [ ] Add integration test with test repository

### 11.8 CLI Integration for History (src/cli.rs)
- [ ] Add history subcommand to CLI
- [ ] Add `--since` option for date filtering
- [ ] Add `--until` option for date range end
- [ ] Add `--last N` option for last N days
- [ ] Add `--by-day` flag for daily breakdown
- [ ] Add `--by-week` flag for weekly aggregation
- [ ] Add `--author` option for author filtering
- [ ] Parse and validate date formats
- [ ] Add unit tests for date parsing

### 11.9 History Output Formatting (src/output.rs)
- [ ] Implement `format_history_table()` for daily stats
- [ ] Create table with columns: Date, Added, Deleted, Net, Languages
- [ ] Format positive numbers with + prefix
- [ ] Format negative numbers with - prefix
- [ ] Use colors for additions (green) and deletions (red)
- [ ] Implement `format_author_stats()` for per-author breakdown
- [ ] Add simple ASCII chart for trend visualization
- [ ] Add unit tests for formatting

### 11.10 Integration
- [ ] Update main.rs to handle history subcommand
- [ ] Detect if path is in a git repository
- [ ] If history mode requested, run GitAnalyzer
- [ ] If not a git repo and history requested, show error
- [ ] Format and display history results
- [ ] Add integration tests for history command

---

## Phase 12: Optimization and Polish

### 12.1 Performance Optimization
- [ ] Profile sniffy on large codebase
- [ ] Identify bottlenecks
- [ ] Add Rayon dependency for parallel processing
- [ ] Parallelize file processing using rayon::par_iter
- [ ] Add thread-safe statistics collection (Mutex<ProjectStats>)
- [ ] Benchmark single-threaded vs parallel
- [ ] Add CLI flag `--jobs N` to control parallelism
- [ ] Optimize hot paths identified in profiling
- [ ] Add benchmarks using criterion crate

### 12.2 String Literal Detection (Phase 2/V2 Feature)
- [ ] Research string literal parsing for each language
- [ ] Create `StringAwareClassifier` for languages with complex strings
- [ ] Implement proper string parsing for JavaScript/TypeScript
- [ ] Handle escape sequences in strings
- [ ] Handle template literals in JavaScript
- [ ] Implement for Python (handle triple quotes, f-strings)
- [ ] Implement for Rust (handle raw strings)
- [ ] Make this optional/experimental feature
- [ ] Add comprehensive tests for string detection
- [ ] Document limitations and trade-offs

### 12.3 Additional Languages
- [ ] Add Kotlin support (.kt, .kts)
- [ ] Add Swift support (.swift)
- [ ] Add Scala support (.scala)
- [ ] Add Elixir support (.ex, .exs)
- [ ] Add Erlang support (.erl)
- [ ] Add Haskell support (.hs)
- [ ] Add Lua support (.lua)
- [ ] Add Perl support (.pl, .pm)
- [ ] Add R support (.r, .R)
- [ ] Add SQL support (.sql)
- [ ] Add Dockerfile support (Dockerfile, *.dockerfile)
- [ ] Add Makefile support (Makefile, *.mk)
- [ ] Add TOML support (.toml)
- [ ] Add INI support (.ini, .cfg)
- [ ] Add Vim script support (.vim)
- [ ] Add Bash/Zsh support improvements
- [ ] Test each language with fixtures

### 12.4 Colorized Output
- [ ] Add dependency: `colored = "2"` or `termcolor = "1"`
- [ ] Add color to table headers
- [ ] Color-code numbers (green for high, red for low)
- [ ] Add CLI flag `--no-color` to disable colors
- [ ] Respect NO_COLOR environment variable
- [ ] Test color output in different terminals

### 12.5 Configuration File Support
- [ ] Design .sniffyrc configuration format (TOML)
- [ ] Add dependency: `serde = { version = "1", features = ["derive"] }`
- [ ] Add dependency: `toml = "0.8"`
- [ ] Define configuration struct
- [ ] Support exclude/include patterns in config
- [ ] Support custom language definitions
- [ ] Support default CLI options
- [ ] Implement config file loading from .sniffyrc
- [ ] Search for config in current dir, then parent dirs
- [ ] CLI args should override config file settings
- [ ] Add config file validation
- [ ] Add example .sniffyrc to repository

---

## Phase 13: Documentation and Polish

### 13.1 Documentation
- [ ] Write comprehensive README.md
  - [ ] Installation instructions
  - [ ] Basic usage examples
  - [ ] CLI reference for all options
  - [ ] Examples for common use cases
  - [ ] Comparison with other tools
  - [ ] Contributing guide
- [ ] Write CHANGELOG.md
- [ ] Add inline code documentation for all public APIs
- [ ] Add doc comments to all structs and impl blocks
- [ ] Generate and review rustdoc output
- [ ] Create examples/ directory with usage examples
- [ ] Add LICENSE file (choose appropriate license)

### 13.2 CLI Help and UX
- [ ] Improve CLI help text with examples
- [ ] Add version information
- [ ] Add `--help` for detailed help
- [ ] Add helpful error messages with suggestions
- [ ] Add "did you mean?" suggestions for typos
- [ ] Test all error paths and messages

### 13.3 Packaging
- [ ] Set up proper Cargo.toml metadata for publishing
- [ ] Add keywords and categories
- [ ] Add repository and documentation URLs
- [ ] Create build script if needed
- [ ] Test `cargo build --release`
- [ ] Test installation with `cargo install --path .`
- [ ] Create GitHub releases workflow (optional)
- [ ] Add installation instructions for various platforms

### 13.4 Final Testing
- [ ] Run full test suite
- [ ] Run clippy and fix all warnings: `cargo clippy -- -D warnings`
- [ ] Run rustfmt: `cargo fmt`
- [ ] Check for unused dependencies: `cargo machete` or similar
- [ ] Test on multiple platforms
- [ ] Perform security audit: `cargo audit`
- [ ] Profile memory usage
- [ ] Profile performance on large repos
- [ ] Compare output with tokei/cloc for validation

---

## Phase 14: Future Enhancements (Post-V1)

### 14.1 JSON/CSV Output
- [ ] Add dependency: `serde_json = "1"`
- [ ] Add dependency: `csv = "1"`
- [ ] Add CLI flag `--format json|csv|table`
- [ ] Implement JSON serialization for ProjectStats
- [ ] Implement CSV output format
- [ ] Add tests for each output format

### 14.2 Web Dashboard
- [ ] Design web dashboard mockup
- [ ] Choose web framework (actix-web, axum, etc.)
- [ ] Create REST API endpoints
- [ ] Implement real-time analysis endpoint
- [ ] Create frontend with charts (Chart.js, D3.js)
- [ ] Add historical trend charts
- [ ] Deploy as separate binary or feature flag

### 14.3 Watch Mode
- [ ] Add dependency: `notify = "6"` for file watching
- [ ] Implement file system watcher
- [ ] Re-analyze on file changes
- [ ] Debounce rapid changes
- [ ] Add CLI flag `--watch`

### 14.4 Plugin System
- [ ] Design plugin API
- [ ] Support custom language definitions via plugins
- [ ] Support custom output formatters
- [ ] Support custom file processors
- [ ] Create plugin loading mechanism

---

## Summary

**Total Tasks: ~300+**

This plan covers everything from initial project setup through advanced features. Each task is designed to be implemented sequentially, building on previous work. The plan is structured to deliver a working MVP early (by end of Phase 9) while leaving room for advanced features in later phases.

**Estimated Implementation Time:**
- Phase 1-9 (MVP): 2-3 weeks full-time
- Phase 10 (Testing): 3-5 days
- Phase 11 (Git History): 1-2 weeks
- Phase 12-13 (Polish): 1 week
- Phase 14 (Future): TBD

**Key Milestones:**
1. End of Phase 4: Can classify lines correctly
2. End of Phase 9: Working CLI tool for snapshot analysis
3. End of Phase 11: Full git history analysis
4. End of Phase 13: Production-ready v1.0
