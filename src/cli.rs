//! Command-line interface argument parsing.
//!
//! This module defines the CLI structure and handles
//! parsing and validation of command-line arguments.

use chrono::{DateTime, Utc};
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

    /// Analyze git commit history
    #[arg(long)]
    pub history: bool,

    /// Only analyze commits since this date (format: YYYY-MM-DD or RFC3339)
    #[arg(long, value_name = "DATE")]
    pub since: Option<String>,

    /// Group history by day (default)
    #[arg(long)]
    pub by_day: bool,

    /// Group history by week
    #[arg(long)]
    pub by_week: bool,

    /// Filter commits by author name
    #[arg(long, value_name = "NAME")]
    pub author: Option<String>,

    /// Output format (table, json, or csv)
    #[arg(long, default_value = "table", value_name = "FORMAT")]
    pub format: String,
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

        // Validate that --by-day and --by-week are mutually exclusive
        if self.by_day && self.by_week {
            return Err("Cannot use both --by-day and --by-week".to_string());
        }

        // Validate that history-related flags require --history
        if !self.history
            && (self.since.is_some() || self.by_day || self.by_week || self.author.is_some())
        {
            return Err(
                "History-related flags (--since, --by-day, --by-week, --author) require --history"
                    .to_string(),
            );
        }

        // Validate format
        let format_lower = self.format.to_lowercase();
        if !["table", "json", "csv"].contains(&format_lower.as_str()) {
            return Err(format!(
                "Invalid format '{}'. Supported formats: table, json, csv",
                self.format
            ));
        }

        Ok(())
    }

    /// Parse the --since date string into a DateTime<Utc>.
    pub fn parse_since_date(&self) -> Result<Option<DateTime<Utc>>, String> {
        let Some(since_str) = &self.since else {
            return Ok(None);
        };

        // Try to parse as RFC3339 first
        if let Ok(dt) = DateTime::parse_from_rfc3339(since_str) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }

        // Try to parse as YYYY-MM-DD
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(since_str, "%Y-%m-%d") {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(Some(naive_datetime.and_utc()));
        }

        Err(format!(
            "Invalid date format '{}'. Use YYYY-MM-DD or RFC3339 format.",
            since_str
        ))
    }
}
