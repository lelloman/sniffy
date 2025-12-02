//! Command-line interface argument parsing.
//!
//! This module defines the CLI structure and handles
//! parsing and validation of command-line arguments.

use clap::Parser;
use std::path::PathBuf;

/// A fast command-line utility for analyzing source code statistics.
#[derive(Parser, Debug)]
#[command(name = "sniffy")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Paths to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Include hidden files and directories
    #[arg(short = 'H', long)]
    pub hidden: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Parse CLI arguments from command line.
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Validate the parsed arguments.
    pub fn validate(&self) -> Result<(), String> {
        for path in &self.paths {
            if !path.exists() {
                return Err(format!("Path does not exist: {}", path.display()));
            }
        }
        Ok(())
    }
}
