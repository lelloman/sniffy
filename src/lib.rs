//! # Sniffy
//!
//! A blazingly fast, parallel source code analyzer for counting lines of code.
//!
//! Sniffy provides accurate classification of source code lines into blank, comment,
//! and code categories across 33+ programming languages. It also includes git history
//! analysis capabilities for tracking code evolution over time.
//!
//! ## Features
//!
//! - **Accurate Line Classification**: Distinguishes code, comments (single and multi-line), and blank lines
//! - **33+ Languages**: JavaScript, TypeScript, Rust, Python, Go, Java, C/C++, and many more
//! - **Parallel Processing**: Utilizes all CPU cores for maximum performance
//! - **Git History Analysis**: Track code changes over time with daily/weekly aggregation
//! - **Multiple Output Formats**: Beautiful tables, JSON, or CSV
//! - **Smart Filtering**: Respects `.gitignore` and skips common build artifacts
//!
//! ## Quick Start
//!
//! ```no_run
//! use sniffy::processor::FileProcessor;
//! use sniffy::stats::ProjectStats;
//! use sniffy::walker::DirectoryWalker;
//! use std::path::Path;
//!
//! // Analyze a directory
//! let processor = FileProcessor::new();
//! let mut stats = ProjectStats::new();
//!
//! let walker = DirectoryWalker::new(Path::new("."));
//! for file_path in walker.walk() {
//!     if let Some((language, file_stats)) = processor.process_file(&file_path) {
//!         stats.add_file_stats(&language, file_stats);
//!     }
//! }
//!
//! // Get totals
//! let (total_files, total_stats) = stats.total();
//! println!("Total files: {}", total_files);
//! println!("Lines of code: {}", total_stats.code);
//! ```
//!
//! ## Modules
//!
//! - [`classifier`]: Line classification engine for determining line types
//! - [`cli`]: Command-line interface definitions and argument parsing
//! - [`error`]: Error types and handling
//! - [`git`]: Git repository analysis and history tracking
//! - [`language`]: Language definitions and file extension detection
//! - [`output`]: Output formatting (tables, JSON, CSV)
//! - [`processor`]: File processing and binary file detection
//! - [`stats`]: Statistics data structures and aggregation
//! - [`walker`]: Directory traversal with .gitignore support
//!
//! ## Examples
//!
//! ### Analyze a specific file
//!
//! ```no_run
//! use sniffy::processor::FileProcessor;
//! use std::path::Path;
//!
//! let processor = FileProcessor::new();
//! if let Some((language, stats)) = processor.process_file(Path::new("src/main.rs")) {
//!     println!("Language: {}", language);
//!     println!("Code lines: {}", stats.code);
//!     println!("Comment lines: {}", stats.comment);
//!     println!("Blank lines: {}", stats.blank);
//! }
//! ```
//!
//! ### Git history analysis
//!
//! ```no_run
//! use sniffy::git::GitAnalyzer;
//!
//! if let Ok(analyzer) = GitAnalyzer::new(".") {
//!     if let Ok(history) = analyzer.analyze_history(None, None, false) {
//!         println!("Total commits: {}", history.total_commits);
//!         println!("Daily stats: {} days", history.daily.len());
//!     }
//! }
//! ```

pub mod classifier;
pub mod cli;
pub mod error;
pub mod git;
pub mod language;
pub mod output;
pub mod processor;
pub mod stats;
pub mod walker;
