//! Git repository analysis and history tracking.
//!
//! This module provides functionality for analyzing git commit history
//! to track code changes over time.

use crate::classifier::LineType;
use crate::stats::FileStats;
use chrono::{DateTime, NaiveDate, Utc};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Daily statistics for git commits.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: NaiveDate,
    pub additions: FileStats,
    pub deletions: FileStats,
    pub net_code: i64,
}

/// Historical statistics aggregated from git history.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HistoricalStats {
    pub daily: Vec<DailyStats>,
    pub by_author: HashMap<String, FileStats>,
    pub total_commits: usize,
}

impl HistoricalStats {
    /// Aggregate daily statistics by week (Monday-Sunday).
    /// Returns a new vector of DailyStats where each entry represents a week.
    pub fn aggregate_by_week(&self) -> Vec<DailyStats> {
        use chrono::Datelike;

        if self.daily.is_empty() {
            return Vec::new();
        }

        let mut weekly: HashMap<(i32, u32), DailyStats> = HashMap::new();

        for daily in &self.daily {
            // Get the ISO week number and year
            let iso_week = daily.date.iso_week();
            let year = iso_week.year();
            let week = iso_week.week();
            let key = (year, week);

            // Get the Monday of this week as the representative date
            let week_start = chrono::NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon)
                .unwrap_or(daily.date);

            let week_stat = weekly.entry(key).or_insert_with(|| DailyStats {
                date: week_start,
                additions: FileStats::default(),
                deletions: FileStats::default(),
                net_code: 0,
            });

            week_stat.additions += daily.additions;
            week_stat.deletions += daily.deletions;
            week_stat.net_code += daily.net_code;
        }

        // Convert to sorted vec
        let mut result: Vec<_> = weekly.into_values().collect();
        result.sort_by(|a, b| b.date.cmp(&a.date)); // Most recent first
        result
    }
}

/// Git repository analyzer.
pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    /// Create a new GitAnalyzer for the given path.
    ///
    /// Returns None if the path is not in a git repository.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    /// Check if a path is in a git repository.
    pub fn is_git_repo<P: AsRef<Path>>(path: P) -> bool {
        Repository::discover(path).is_ok()
    }

    /// Analyze commit history and return historical statistics.
    pub fn analyze_history(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<HistoricalStats, git2::Error> {
        let mut stats = HistoricalStats::default();
        let mut daily_map: HashMap<NaiveDate, DailyStats> = HashMap::new();

        // Walk commits
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            // Filter by date if specified
            let commit_time = DateTime::from_timestamp(commit.time().seconds(), 0)
                .unwrap_or(DateTime::UNIX_EPOCH);

            if let Some(since_date) = since {
                if commit_time < since_date {
                    break; // Stop processing older commits
                }
            }

            stats.total_commits += 1;

            // Get commit date
            let date = commit_time.date_naive();

            // Analyze commit diff
            let (additions, deletions) = self.analyze_commit(&commit)?;

            // Update daily stats
            let daily_stat = daily_map.entry(date).or_insert_with(|| DailyStats {
                date,
                additions: FileStats::default(),
                deletions: FileStats::default(),
                net_code: 0,
            });

            daily_stat.additions += additions;
            daily_stat.deletions += deletions;
            daily_stat.net_code += (additions.code as i64) - (deletions.code as i64);

            // Track by author - extract name to owned String to avoid lifetime issues
            let author_name = commit.author().name().map(|s| s.to_string());
            if let Some(author) = author_name {
                let author_stats = stats
                    .by_author
                    .entry(author)
                    .or_insert_with(FileStats::default);
                *author_stats += additions;
            }
        }

        // Convert daily map to sorted vec
        let mut daily: Vec<_> = daily_map.into_values().collect();
        daily.sort_by(|a, b| b.date.cmp(&a.date)); // Most recent first

        stats.daily = daily;
        Ok(stats)
    }

    /// Analyze a single commit and return added/deleted line stats.
    fn analyze_commit(&self, commit: &git2::Commit) -> Result<(FileStats, FileStats), git2::Error> {
        let mut additions = FileStats::default();
        let mut deletions = FileStats::default();

        // Get the tree for this commit
        let tree = commit.tree()?;

        // Get parent tree (if exists)
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        // Create diff
        let diff = if let Some(parent_tree) = parent_tree {
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?
        } else {
            // First commit - diff against empty tree
            self.repo.diff_tree_to_tree(None, Some(&tree), None)?
        };

        // Process diff
        diff.foreach(
            &mut |_delta, _progress| {
                // Continue processing all files
                true
            },
            None,
            None,
            Some(&mut |_delta, _hunk, line| {
                let line_type = Self::classify_diff_line(line.content());

                match line.origin() {
                    '+' => {
                        // Added line
                        match line_type {
                            LineType::Blank => additions.blank += 1,
                            LineType::Comment => additions.comment += 1,
                            LineType::Code => additions.code += 1,
                        }
                    }
                    '-' => {
                        // Deleted line
                        match line_type {
                            LineType::Blank => deletions.blank += 1,
                            LineType::Comment => deletions.comment += 1,
                            LineType::Code => deletions.code += 1,
                        }
                    }
                    _ => {} // Context lines, ignore
                }
                true
            }),
        )?;

        Ok((additions, deletions))
    }

    /// Classify a single line from a diff.
    fn classify_diff_line(content: &[u8]) -> LineType {
        // Convert to string, skip invalid UTF-8
        let line = match std::str::from_utf8(content) {
            Ok(s) => s,
            Err(_) => return LineType::Code, // Treat binary as code
        };

        let trimmed = line.trim();

        // Check if blank
        if trimmed.is_empty() {
            return LineType::Blank;
        }

        // Simple heuristics for comments (not perfect, but reasonable)
        // This is a simplified version - doesn't track multi-line comment state
        if trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("--")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*/")
            || trimmed.starts_with('*')
            || trimmed.starts_with("<!--")
            || trimmed.starts_with("-->")
        {
            return LineType::Comment;
        }

        LineType::Code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_diff_line_blank() {
        assert_eq!(GitAnalyzer::classify_diff_line(b""), LineType::Blank);
        assert_eq!(GitAnalyzer::classify_diff_line(b"   "), LineType::Blank);
        assert_eq!(GitAnalyzer::classify_diff_line(b"\t\t"), LineType::Blank);
    }

    #[test]
    fn test_classify_diff_line_comment() {
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"// comment"),
            LineType::Comment
        );
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"# comment"),
            LineType::Comment
        );
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"/* comment"),
            LineType::Comment
        );
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"-- SQL comment"),
            LineType::Comment
        );
    }

    #[test]
    fn test_classify_diff_line_code() {
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"let x = 5;"),
            LineType::Code
        );
        assert_eq!(
            GitAnalyzer::classify_diff_line(b"fn main() {"),
            LineType::Code
        );
    }

    #[test]
    fn test_is_git_repo() {
        // Current directory should be a git repo (sniffy project)
        assert!(GitAnalyzer::is_git_repo("."));
    }
}
