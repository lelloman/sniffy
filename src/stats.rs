//! Statistics data structures and aggregation.
//!
//! This module defines data structures for tracking code statistics
//! and methods for aggregating them across files and languages.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

/// Statistics for a single file or aggregated files.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStats {
    /// Number of blank lines.
    pub blank: usize,
    /// Number of comment lines.
    pub comment: usize,
    /// Number of code lines.
    pub code: usize,
}

impl FileStats {
    /// Create a new FileStats with all counts set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate the total number of lines.
    pub fn total(&self) -> usize {
        self.blank + self.comment + self.code
    }
}

impl Add for FileStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            blank: self.blank + other.blank,
            comment: self.comment + other.comment,
            code: self.code + other.code,
        }
    }
}

impl AddAssign for FileStats {
    fn add_assign(&mut self, other: Self) {
        self.blank += other.blank;
        self.comment += other.comment;
        self.code += other.code;
    }
}

/// Statistics for a specific programming language.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageStats {
    /// Name of the language.
    pub language: String,
    /// Number of files in this language.
    pub files: usize,
    /// Aggregated statistics for all files.
    pub stats: FileStats,
}

/// Statistics for an entire project.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectStats {
    /// Map from language name to language statistics.
    languages: HashMap<String, LanguageStats>,
}

impl ProjectStats {
    /// Create a new empty ProjectStats.
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
        }
    }

    /// Add file statistics for a specific language.
    pub fn add_file_stats(&mut self, language: &str, stats: FileStats) {
        let lang_stats = self
            .languages
            .entry(language.to_string())
            .or_insert_with(|| LanguageStats {
                language: language.to_string(),
                files: 0,
                stats: FileStats::default(),
            });
        lang_stats.files += 1;
        lang_stats.stats += stats;
    }

    /// Get a sorted list of languages.
    pub fn get_languages(&self) -> Vec<&LanguageStats> {
        let mut languages: Vec<_> = self.languages.values().collect();
        languages.sort_by(|a, b| a.language.cmp(&b.language));
        languages
    }

    /// Calculate total statistics across all languages.
    pub fn total(&self) -> (usize, FileStats) {
        let mut total_files = 0;
        let mut total_stats = FileStats::default();

        for lang_stats in self.languages.values() {
            total_files += lang_stats.files;
            total_stats += lang_stats.stats;
        }

        (total_files, total_stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_stats_new() {
        let stats = FileStats::new();
        assert_eq!(stats.blank, 0);
        assert_eq!(stats.comment, 0);
        assert_eq!(stats.code, 0);
    }

    #[test]
    fn test_file_stats_total() {
        let stats = FileStats {
            blank: 10,
            comment: 20,
            code: 70,
        };
        assert_eq!(stats.total(), 100);
    }

    #[test]
    fn test_file_stats_add() {
        let stats1 = FileStats {
            blank: 10,
            comment: 20,
            code: 30,
        };
        let stats2 = FileStats {
            blank: 5,
            comment: 15,
            code: 25,
        };
        let result = stats1 + stats2;
        assert_eq!(result.blank, 15);
        assert_eq!(result.comment, 35);
        assert_eq!(result.code, 55);
    }

    #[test]
    fn test_file_stats_add_assign() {
        let mut stats1 = FileStats {
            blank: 10,
            comment: 20,
            code: 30,
        };
        let stats2 = FileStats {
            blank: 5,
            comment: 15,
            code: 25,
        };
        stats1 += stats2;
        assert_eq!(stats1.blank, 15);
        assert_eq!(stats1.comment, 35);
        assert_eq!(stats1.code, 55);
    }

    #[test]
    fn test_project_stats_new() {
        let stats = ProjectStats::new();
        assert_eq!(stats.languages.len(), 0);
    }

    #[test]
    fn test_project_stats_add_file_stats() {
        let mut project = ProjectStats::new();
        project.add_file_stats(
            "Rust",
            FileStats {
                blank: 10,
                comment: 20,
                code: 70,
            },
        );
        project.add_file_stats(
            "Rust",
            FileStats {
                blank: 5,
                comment: 10,
                code: 35,
            },
        );

        let rust_stats = &project.languages["Rust"];
        assert_eq!(rust_stats.files, 2);
        assert_eq!(rust_stats.stats.blank, 15);
        assert_eq!(rust_stats.stats.comment, 30);
        assert_eq!(rust_stats.stats.code, 105);
    }

    #[test]
    fn test_project_stats_total() {
        let mut project = ProjectStats::new();
        project.add_file_stats(
            "Rust",
            FileStats {
                blank: 10,
                comment: 20,
                code: 70,
            },
        );
        project.add_file_stats(
            "Python",
            FileStats {
                blank: 5,
                comment: 10,
                code: 35,
            },
        );

        let (total_files, total_stats) = project.total();
        assert_eq!(total_files, 2);
        assert_eq!(total_stats.blank, 15);
        assert_eq!(total_stats.comment, 30);
        assert_eq!(total_stats.code, 105);
    }

    #[test]
    fn test_project_stats_get_languages() {
        let mut project = ProjectStats::new();
        project.add_file_stats("Rust", FileStats::default());
        project.add_file_stats("Python", FileStats::default());
        project.add_file_stats("JavaScript", FileStats::default());

        let languages = project.get_languages();
        assert_eq!(languages.len(), 3);
        // Should be sorted alphabetically
        assert_eq!(languages[0].language, "JavaScript");
        assert_eq!(languages[1].language, "Python");
        assert_eq!(languages[2].language, "Rust");
    }
}
