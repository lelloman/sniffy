//! Output formatting and display.
//!
//! This module handles formatting statistics as tables
//! and other output formats for the terminal.

use crate::git::HistoricalStats;
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

    /// Format a signed number with thousand separators and +/- sign.
    fn format_signed_number(n: i64) -> String {
        let sign = if n >= 0 { "+" } else { "" };
        let s = n.abs().to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }

        format!("{}{}", sign, result)
    }

    /// Format git history statistics as a table.
    /// The `period_label` parameter can be "Daily" or "Weekly".
    pub fn format_history(
        stats: &HistoricalStats,
        time_series: &[crate::git::DailyStats],
        period_label: &str,
        limit: Option<usize>,
    ) -> String {
        let mut output = String::new();

        // Summary
        output.push_str(&format!(
            "Git History Analysis\n\
             Total Commits: {}\n\
             Date Range: {} to {}\n\n",
            Self::format_number(stats.total_commits),
            time_series
                .last()
                .map(|d| d.date.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            time_series
                .first()
                .map(|d| d.date.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        ));

        // Time series statistics table
        if !time_series.is_empty() {
            output.push_str(&format!("{} Statistics:\n", period_label));
            let mut table = Table::new();

            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);

            table.set_header(vec![
                Cell::new("Date").fg(Color::Cyan),
                Cell::new("Added").fg(Color::Cyan),
                Cell::new("Deleted").fg(Color::Cyan),
                Cell::new("Net Change").fg(Color::Cyan),
            ]);

            // Limit the number of rows if specified
            let rows_to_show = limit.unwrap_or(time_series.len()).min(time_series.len());

            for daily in time_series.iter().take(rows_to_show) {
                let net_code = daily.net_code;
                let net_cell = Cell::new(Self::format_signed_number(net_code));
                let net_cell = if net_code > 0 {
                    net_cell.fg(Color::Green)
                } else if net_code < 0 {
                    net_cell.fg(Color::Red)
                } else {
                    net_cell
                };

                table.add_row(vec![
                    Cell::new(daily.date.to_string()),
                    Cell::new(Self::format_number(daily.additions.code)).fg(Color::Green),
                    Cell::new(Self::format_number(daily.deletions.code)).fg(Color::Red),
                    net_cell,
                ]);
            }

            output.push_str(&table.to_string());
            output.push('\n');

            if time_series.len() > rows_to_show {
                let period_name = if period_label == "Weekly" {
                    "weeks"
                } else {
                    "days"
                };
                output.push_str(&format!(
                    "... and {} more {}\n\n",
                    time_series.len() - rows_to_show,
                    period_name
                ));
            }
        }

        // Author statistics
        if !stats.by_author.is_empty() {
            output.push_str("Top Contributors:\n");
            let mut table = Table::new();

            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);

            table.set_header(vec![
                Cell::new("Author").fg(Color::Cyan),
                Cell::new("Code Lines").fg(Color::Cyan),
                Cell::new("Comments").fg(Color::Cyan),
                Cell::new("Total").fg(Color::Cyan),
            ]);

            // Sort authors by total lines contributed
            let mut authors: Vec<_> = stats.by_author.iter().collect();
            authors.sort_by(|a, b| {
                let total_a = a.1.code + a.1.comment + a.1.blank;
                let total_b = b.1.code + b.1.comment + b.1.blank;
                total_b.cmp(&total_a)
            });

            for (author, author_stats) in authors.iter().take(10) {
                table.add_row(vec![
                    Cell::new(author),
                    Cell::new(Self::format_number(author_stats.code)),
                    Cell::new(Self::format_number(author_stats.comment)),
                    Cell::new(Self::format_number(author_stats.total())),
                ]);
            }

            output.push_str(&table.to_string());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::FileStats;

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
