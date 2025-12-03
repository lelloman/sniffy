# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive rustdoc documentation for public APIs
- Configuration file support (.sniffyrc)
- Generated file detection (--skip-generated flag)
- Additional CLI options: --exclude, --languages, --sort-by
- Progress indicator improvements
- NO_COLOR environment variable support

## [0.1.0] - 2025-12-03

### Added
- Initial release of Sniffy
- Source code line counting with accurate blank/comment/code classification
- Support for 33+ programming languages (JavaScript, TypeScript, Rust, Python, Go, Java, C/C++, C#, Ruby, PHP, Shell, Kotlin, Swift, Scala, Haskell, Elixir, Erlang, Lua, Perl, R, SQL, HTML, CSS, SCSS, Markdown, JSON, YAML, XML, TOML, INI, Vim Script)
- Multi-line comment detection with state tracking
- Git history analysis with --history flag
- Daily and weekly aggregation (--by-day, --by-week)
- Date filtering with --since option
- Author filtering with --author option
- Top contributors statistics
- Multiple output formats: table (default), JSON, CSV
- Parallel processing with Rayon for high performance
- Configurable parallelism with --jobs flag (0 = auto-detect CPUs)
- Smart file filtering:
  - Automatic .gitignore respect
  - Binary file detection and skipping
  - Common build artifacts (node_modules, target, build, dist)
  - Minified files (.min.js, .min.css)
  - Lock files (package-lock.json, Cargo.lock, etc.)
- Hidden file handling with --hidden flag
- Verbose mode with --verbose flag
- Beautiful colored table output using comfy-table
- Comprehensive test suite (96 tests: 64 unit + 32 integration)
- Test fixtures covering edge cases and multi-language projects

### Performance
- Parallel processing across all CPU cores
- Optimized for projects from 10 to 10,000+ files
- Linear scaling with available CPU cores

### Documentation
- Comprehensive README.md with examples
- Complete DESIGN.md architecture documentation
- Detailed PLAN.md implementation roadmap
- Test fixture documentation

[Unreleased]: https://github.com/yourusername/sniffy/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/sniffy/releases/tag/v0.1.0
