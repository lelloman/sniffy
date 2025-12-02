//! Output formatting and display.
//!
//! This module handles formatting statistics as tables
//! and other output formats for the terminal.

use crate::stats::ProjectStats;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};

/// Output formatter for displaying statistics.
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format project statistics as a table.
    pub fn format_table(stats: &ProjectStats) -> String {
        let mut table = Table::new();

        // Set table style
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Add header
        table.set_header(vec![
            Cell::new("Language").fg(Color::Cyan),
            Cell::new("Files").fg(Color::Cyan),
            Cell::new("Blank").fg(Color::Cyan),
            Cell::new("Comment").fg(Color::Cyan),
            Cell::new("Code").fg(Color::Cyan),
            Cell::new("Total").fg(Color::Cyan),
        ]);

        // Add rows for each language
        let languages = stats.get_languages();
        for lang_stats in &languages {
            table.add_row(vec![
                Cell::new(&lang_stats.language),
                Cell::new(Self::format_number(lang_stats.files)),
                Cell::new(Self::format_number(lang_stats.stats.blank)),
                Cell::new(Self::format_number(lang_stats.stats.comment)),
                Cell::new(Self::format_number(lang_stats.stats.code)),
                Cell::new(Self::format_number(lang_stats.stats.total())),
            ]);
        }

        // Add total row
        let (total_files, total_stats) = stats.total();
        if !languages.is_empty() {
            table.add_row(vec![
                Cell::new("Total").fg(Color::Green),
                Cell::new(Self::format_number(total_files)).fg(Color::Green),
                Cell::new(Self::format_number(total_stats.blank)).fg(Color::Green),
                Cell::new(Self::format_number(total_stats.comment)).fg(Color::Green),
                Cell::new(Self::format_number(total_stats.code)).fg(Color::Green),
                Cell::new(Self::format_number(total_stats.total())).fg(Color::Green),
            ]);
        }

        table.to_string()
    }

    /// Format a number with thousand separators.
    fn format_number(n: usize) -> String {
        let s = n.to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(OutputFormatter::format_number(0), "0");
        assert_eq!(OutputFormatter::format_number(1), "1");
        assert_eq!(OutputFormatter::format_number(10), "10");
        assert_eq!(OutputFormatter::format_number(100), "100");
        assert_eq!(OutputFormatter::format_number(1000), "1,000");
        assert_eq!(OutputFormatter::format_number(1234), "1,234");
        assert_eq!(OutputFormatter::format_number(12345), "12,345");
        assert_eq!(OutputFormatter::format_number(123456), "123,456");
        assert_eq!(OutputFormatter::format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_format_table_empty() {
        let stats = ProjectStats::new();
        let table = OutputFormatter::format_table(&stats);

        // Should have header but no data rows
        assert!(table.contains("Language"));
        assert!(table.contains("Files"));
    }

    #[test]
    fn test_format_table_with_data() {
        let mut stats = ProjectStats::new();
        stats.add_file_stats(
            "Rust",
            FileStats {
                blank: 10,
                comment: 20,
                code: 70,
            },
        );
        stats.add_file_stats(
            "Python",
            FileStats {
                blank: 5,
                comment: 10,
                code: 35,
            },
        );

        let table = OutputFormatter::format_table(&stats);

        // Check for language names
        assert!(table.contains("Rust") || table.contains("Python"));
        // Check for Total row
        assert!(table.contains("Total"));
        // Check for some numbers
        assert!(table.contains("70"));
        assert!(table.contains("35"));
    }
}
