//! Output formatting and display.
//!
//! This module handles formatting statistics as tables
//! and other output formats for the terminal.

use crate::git::{DailyStats, HistoricalStats};
use crate::stats::ProjectStats;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use serde::Serialize;

/// Output formatter for displaying statistics.
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format project statistics as a table.
    ///
    /// If `use_color` is false, colors will be disabled.
    pub fn format_table(stats: &ProjectStats, use_color: bool) -> String {
        let mut table = Table::new();

        // Set table style
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Add header
        let header_cells = vec!["Language", "Files", "Blank", "Comment", "Code", "Total"];
        if use_color {
            table.set_header(
                header_cells
                    .into_iter()
                    .map(|h| Cell::new(h).fg(Color::Cyan))
                    .collect::<Vec<_>>(),
            );
        } else {
            table.set_header(header_cells);
        }

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
            // Create owned strings for the numbers
            let total_files_str = Self::format_number(total_files);
            let total_blank_str = Self::format_number(total_stats.blank);
            let total_comment_str = Self::format_number(total_stats.comment);
            let total_code_str = Self::format_number(total_stats.code);
            let total_total_str = Self::format_number(total_stats.total());

            if use_color {
                table.add_row(vec![
                    Cell::new("Total").fg(Color::Green),
                    Cell::new(total_files_str).fg(Color::Green),
                    Cell::new(total_blank_str).fg(Color::Green),
                    Cell::new(total_comment_str).fg(Color::Green),
                    Cell::new(total_code_str).fg(Color::Green),
                    Cell::new(total_total_str).fg(Color::Green),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new("Total"),
                    Cell::new(total_files_str),
                    Cell::new(total_blank_str),
                    Cell::new(total_comment_str),
                    Cell::new(total_code_str),
                    Cell::new(total_total_str),
                ]);
            }
        }

        table.to_string()
    }

    /// Format a number with thousand separators.
    fn format_number(n: usize) -> String {
        let s = n.to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
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
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(*c);
        }

        format!("{}{}", sign, result)
    }

    /// Format git history statistics as a table.
    /// The `period_label` parameter can be "Daily" or "Weekly".
    ///
    /// If `use_color` is false, colors will be disabled.
    pub fn format_history(
        stats: &HistoricalStats,
        time_series: &[crate::git::DailyStats],
        period_label: &str,
        limit: Option<usize>,
        use_color: bool,
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

            let header_cells = vec!["Date", "Added", "Deleted", "Net Change"];
            if use_color {
                table.set_header(
                    header_cells
                        .into_iter()
                        .map(|h| Cell::new(h).fg(Color::Cyan))
                        .collect::<Vec<_>>(),
                );
            } else {
                table.set_header(header_cells);
            }

            // Limit the number of rows if specified
            let rows_to_show = limit.unwrap_or(time_series.len()).min(time_series.len());

            for daily in time_series.iter().take(rows_to_show) {
                let net_code = daily.net_code;
                let net_cell = Cell::new(Self::format_signed_number(net_code));
                let net_cell = if use_color {
                    if net_code > 0 {
                        net_cell.fg(Color::Green)
                    } else if net_code < 0 {
                        net_cell.fg(Color::Red)
                    } else {
                        net_cell
                    }
                } else {
                    net_cell
                };

                let added_cell = Cell::new(Self::format_number(daily.additions.code));
                let deleted_cell = Cell::new(Self::format_number(daily.deletions.code));

                let added_cell = if use_color {
                    added_cell.fg(Color::Green)
                } else {
                    added_cell
                };
                let deleted_cell = if use_color {
                    deleted_cell.fg(Color::Red)
                } else {
                    deleted_cell
                };

                table.add_row(vec![
                    Cell::new(daily.date.to_string()),
                    added_cell,
                    deleted_cell,
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

            let author_header_cells = vec!["Author", "Code Lines", "Comments", "Total"];
            if use_color {
                table.set_header(
                    author_header_cells
                        .into_iter()
                        .map(|h| Cell::new(h).fg(Color::Cyan))
                        .collect::<Vec<_>>(),
                );
            } else {
                table.set_header(author_header_cells);
            }

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

    /// Format project statistics as JSON.
    pub fn format_json(stats: &ProjectStats) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct JsonOutput {
            languages: Vec<crate::stats::LanguageStats>,
            total_files: usize,
            total_stats: crate::stats::FileStats,
        }

        let languages = stats.get_languages().into_iter().cloned().collect();
        let (total_files, total_stats) = stats.total();

        let output = JsonOutput {
            languages,
            total_files,
            total_stats,
        };

        serde_json::to_string_pretty(&output)
    }

    /// Format git history as JSON.
    pub fn format_history_json(
        stats: &HistoricalStats,
        time_series: &[DailyStats],
        period_label: &str,
    ) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct JsonHistoryOutput {
            total_commits: usize,
            period: String,
            time_series: Vec<DailyStats>,
            by_author: std::collections::HashMap<String, crate::stats::FileStats>,
        }

        let output = JsonHistoryOutput {
            total_commits: stats.total_commits,
            period: period_label.to_lowercase(),
            time_series: time_series.to_vec(),
            by_author: stats.by_author.clone(),
        };

        serde_json::to_string_pretty(&output)
    }

    /// Format project statistics as CSV.
    pub fn format_csv(stats: &ProjectStats) -> String {
        let mut output = String::new();

        // Header
        output.push_str("language,files,blank,comment,code,total\n");

        // Data rows
        let languages = stats.get_languages();
        for lang_stats in &languages {
            output.push_str(&format!(
                "{},{},{},{},{},{}\n",
                lang_stats.language,
                lang_stats.files,
                lang_stats.stats.blank,
                lang_stats.stats.comment,
                lang_stats.stats.code,
                lang_stats.stats.total()
            ));
        }

        // Total row
        let (total_files, total_stats) = stats.total();
        if !languages.is_empty() {
            output.push_str(&format!(
                "Total,{},{},{},{},{}\n",
                total_files,
                total_stats.blank,
                total_stats.comment,
                total_stats.code,
                total_stats.total()
            ));
        }

        output
    }

    /// Format git history as CSV.
    pub fn format_history_csv(
        stats: &HistoricalStats,
        time_series: &[DailyStats],
        period_label: &str,
    ) -> String {
        let mut output = String::new();

        // Summary header
        output.push_str(&format!(
            "# Git History Analysis - {} Statistics\n",
            period_label
        ));
        output.push_str(&format!("# Total Commits: {}\n\n", stats.total_commits));

        // Time series data
        output.push_str("date,additions_code,deletions_code,net_change\n");
        for daily in time_series {
            output.push_str(&format!(
                "{},{},{},{}\n",
                daily.date,
                daily.additions.code,
                daily.deletions.code,
                if daily.net_code >= 0 {
                    format!("+{}", daily.net_code)
                } else {
                    daily.net_code.to_string()
                }
            ));
        }

        // Author statistics
        if !stats.by_author.is_empty() {
            output.push_str("\n# Top Contributors\n");
            output.push_str("author,code_lines,comments,total\n");

            let mut authors: Vec<_> = stats.by_author.iter().collect();
            authors.sort_by(|a, b| {
                let total_a = a.1.code + a.1.comment + a.1.blank;
                let total_b = b.1.code + b.1.comment + b.1.blank;
                total_b.cmp(&total_a)
            });

            for (author, author_stats) in authors.iter().take(10) {
                output.push_str(&format!(
                    "{},{},{},{}\n",
                    author,
                    author_stats.code,
                    author_stats.comment,
                    author_stats.total()
                ));
            }
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
        let table = OutputFormatter::format_table(&stats, true);

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

        let table = OutputFormatter::format_table(&stats, true);

        // Check for language names
        assert!(table.contains("Rust") || table.contains("Python"));
        // Check for Total row
        assert!(table.contains("Total"));
        // Check for some numbers
        assert!(table.contains("70"));
        assert!(table.contains("35"));
    }
}
