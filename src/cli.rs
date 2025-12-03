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

    /// Only analyze commits until this date (format: YYYY-MM-DD or RFC3339)
    #[arg(long, value_name = "DATE")]
    pub until: Option<String>,

    /// Only analyze commits from the last N days
    #[arg(long, value_name = "N", conflicts_with_all = ["since", "until"])]
    pub last: Option<usize>,

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

    /// Number of parallel jobs (0 = number of CPUs)
    #[arg(short = 'j', long, default_value = "0", value_name = "N")]
    pub jobs: usize,
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
            && (self.since.is_some()
                || self.until.is_some()
                || self.last.is_some()
                || self.by_day
                || self.by_week
                || self.author.is_some())
        {
            return Err(
                "History-related flags (--since, --until, --last, --by-day, --by-week, --author) require --history"
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

    /// Parse the --since date string into a `DateTime<Utc>`.
    /// If --last N is specified, calculates the date N days ago.
    pub fn parse_since_date(&self) -> Result<Option<DateTime<Utc>>, String> {
        // Handle --last N days
        if let Some(days) = self.last {
            let now = Utc::now();
            let duration = chrono::Duration::days(days as i64);
            return Ok(Some(now - duration));
        }

        let Some(since_str) = &self.since else {
            return Ok(None);
        };

        Self::parse_date_string(since_str)
    }

    /// Parse the --until date string into a `DateTime<Utc>`.
    pub fn parse_until_date(&self) -> Result<Option<DateTime<Utc>>, String> {
        let Some(until_str) = &self.until else {
            return Ok(None);
        };

        Self::parse_date_string(until_str)
    }

    /// Parse a date string in either RFC3339 or YYYY-MM-DD format.
    fn parse_date_string(date_str: &str) -> Result<Option<DateTime<Utc>>, String> {
        // Try to parse as RFC3339 first
        if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }

        // Try to parse as YYYY-MM-DD
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(Some(naive_datetime.and_utc()));
        }

        Err(format!(
            "Invalid date format '{}'. Use YYYY-MM-DD or RFC3339 format.",
            date_str
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_string_yyyy_mm_dd() {
        let result = Cli::parse_date_string("2024-01-15");
        assert!(result.is_ok());
        let dt = result.unwrap().unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    }

    #[test]
    fn test_parse_date_string_rfc3339() {
        let result = Cli::parse_date_string("2024-01-15T10:30:00Z");
        assert!(result.is_ok());
        let dt = result.unwrap().unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    }

    #[test]
    fn test_parse_date_string_invalid() {
        let result = Cli::parse_date_string("not-a-date");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date format"));
    }

    #[test]
    fn test_parse_date_string_invalid_format() {
        let result = Cli::parse_date_string("01/15/2024");
        assert!(result.is_err());
    }
}
