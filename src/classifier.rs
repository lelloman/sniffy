//! Line classification engine.
//!
//! This module implements the logic for classifying lines as
//! blank, comment, or code based on language syntax rules.

use crate::language::{CommentPair, LanguageInfo};
use crate::stats::FileStats;

/// Type of a line in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    /// Line contains only whitespace.
    Blank,
    /// Line contains only comments.
    Comment,
    /// Line contains code (may also contain comments).
    Code,
}

/// State tracker for multi-line comment processing.
#[derive(Debug, Clone)]
pub struct ClassifierState {
    /// Whether we're currently inside a multi-line comment.
    in_multi_line_comment: bool,
    /// The delimiter pair we're currently inside (if any).
    current_delimiter: Option<CommentPair>,
}

impl ClassifierState {
    /// Create a new ClassifierState.
    pub fn new() -> Self {
        Self {
            in_multi_line_comment: false,
            current_delimiter: None,
        }
    }

    /// Reset the state to initial values.
    pub fn reset(&mut self) {
        self.in_multi_line_comment = false;
        self.current_delimiter = None;
    }
}

impl Default for ClassifierState {
    fn default() -> Self {
        Self::new()
    }
}

/// Line classifier that determines line types based on language rules.
pub struct LineClassifier<'a> {
    language: &'a LanguageInfo,
}

impl<'a> LineClassifier<'a> {
    /// Create a new LineClassifier for the given language.
    pub fn new(language: &'a LanguageInfo) -> Self {
        Self { language }
    }

    /// Trim leading and trailing whitespace from a line.
    fn trim_line(line: &str) -> &str {
        line.trim()
    }

    /// Check if a line is blank (only whitespace).
    fn is_blank(line: &str) -> bool {
        Self::trim_line(line).is_empty()
    }

    /// Check if a line starts with a single-line comment.
    fn starts_with_single_comment(&self, line: &str) -> bool {
        let trimmed = Self::trim_line(line);
        for comment in self.language.single_line_comments {
            if trimmed.starts_with(comment) {
                return true;
            }
        }
        false
    }

    /// Find the position of a multi-line comment start delimiter in a line.
    fn contains_multi_line_start(&self, line: &str) -> Option<(usize, &CommentPair)> {
        for pair in self.language.multi_line_comments {
            if let Some(pos) = line.find(pair.start) {
                return Some((pos, pair));
            }
        }
        None
    }

    /// Find the position of a multi-line comment end delimiter in a line.
    fn contains_multi_line_end(&self, line: &str, delimiter: &CommentPair) -> Option<usize> {
        line.find(delimiter.end).map(|pos| pos + delimiter.end.len())
    }

    /// Classify a single line of code.
    ///
    /// This method updates the state and returns the line type.
    pub fn classify_line(&self, line: &str, state: &mut ClassifierState) -> LineType {
        // Check for blank line first
        if Self::is_blank(line) {
            return LineType::Blank;
        }

        let trimmed = Self::trim_line(line);

        // Handle shebang lines as code
        if trimmed.starts_with("#!") {
            return LineType::Code;
        }

        // If we're already in a multi-line comment
        if state.in_multi_line_comment {
            if let Some(ref delimiter) = state.current_delimiter {
                // Look for the end delimiter
                if let Some(end_pos) = self.contains_multi_line_end(line, delimiter) {
                    // Comment ends on this line
                    state.in_multi_line_comment = false;
                    state.current_delimiter = None;

                    // Check if there's code after the comment end
                    let after_comment = &line[end_pos..];
                    if !Self::is_blank(after_comment) {
                        // There's code after the comment
                        return LineType::Code;
                    } else {
                        return LineType::Comment;
                    }
                } else {
                    // Still in comment, no end delimiter on this line
                    return LineType::Comment;
                }
            }
        }

        // Not in multi-line comment
        // Check for single-line comment
        if self.starts_with_single_comment(trimmed) {
            return LineType::Comment;
        }

        // Check for multi-line comment start
        if let Some((start_pos, pair)) = self.contains_multi_line_start(line) {
            // Check if it also ends on the same line
            let after_start = &line[start_pos + pair.start.len()..];
            if let Some(end_pos_relative) = after_start.find(pair.end) {
                // Multi-line comment starts and ends on same line
                let end_pos = start_pos + pair.start.len() + end_pos_relative + pair.end.len();

                // Check if there's code before or after
                let before_comment = &line[..start_pos];
                let after_comment = &line[end_pos..];

                if !Self::is_blank(before_comment) || !Self::is_blank(after_comment) {
                    return LineType::Code;
                } else {
                    return LineType::Comment;
                }
            } else {
                // Multi-line comment starts but doesn't end
                state.in_multi_line_comment = true;
                state.current_delimiter = Some(pair.clone());

                // Check if there's code before the comment start
                let before_comment = &line[..start_pos];
                if !Self::is_blank(before_comment) {
                    return LineType::Code;
                } else {
                    return LineType::Comment;
                }
            }
        }

        // No comments found, it's code
        LineType::Code
    }
}

/// Classify all lines in a file and return statistics.
pub fn classify_file(lines: &[String], language: &LanguageInfo) -> FileStats {
    let classifier = LineClassifier::new(language);
    let mut state = ClassifierState::new();
    let mut stats = FileStats::new();

    for line in lines {
        match classifier.classify_line(line, &mut state) {
            LineType::Blank => stats.blank += 1,
            LineType::Comment => stats.comment += 1,
            LineType::Code => stats.code += 1,
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::LANGUAGES;

    fn get_rust_language() -> &'static LanguageInfo {
        LANGUAGES.iter().find(|l| l.name == "Rust").unwrap()
    }

    fn get_python_language() -> &'static LanguageInfo {
        LANGUAGES.iter().find(|l| l.name == "Python").unwrap()
    }

    #[test]
    fn test_line_type_equality() {
        assert_eq!(LineType::Blank, LineType::Blank);
        assert_eq!(LineType::Comment, LineType::Comment);
        assert_eq!(LineType::Code, LineType::Code);
        assert_ne!(LineType::Blank, LineType::Code);
    }

    #[test]
    fn test_classifier_state_new() {
        let state = ClassifierState::new();
        assert!(!state.in_multi_line_comment);
        assert!(state.current_delimiter.is_none());
    }

    #[test]
    fn test_classifier_state_reset() {
        let mut state = ClassifierState::new();
        state.in_multi_line_comment = true;
        state.current_delimiter = Some(CommentPair::new("/*", "*/"));

        state.reset();
        assert!(!state.in_multi_line_comment);
        assert!(state.current_delimiter.is_none());
    }

    #[test]
    fn test_classify_blank_lines() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(classifier.classify_line("", &mut state), LineType::Blank);
        assert_eq!(classifier.classify_line("   ", &mut state), LineType::Blank);
        assert_eq!(classifier.classify_line("\t\t", &mut state), LineType::Blank);
        assert_eq!(classifier.classify_line("  \t  ", &mut state), LineType::Blank);
    }

    #[test]
    fn test_classify_single_line_comments() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(
            classifier.classify_line("// this is a comment", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("  // comment with leading spaces", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("/// doc comment", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("//! inner doc comment", &mut state),
            LineType::Comment
        );
    }

    #[test]
    fn test_classify_code_lines() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(
            classifier.classify_line("let x = 5;", &mut state),
            LineType::Code
        );
        assert_eq!(
            classifier.classify_line("fn main() {", &mut state),
            LineType::Code
        );
        assert_eq!(
            classifier.classify_line("    println!(\"hello\");", &mut state),
            LineType::Code
        );
    }

    #[test]
    fn test_classify_mixed_lines() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Code with trailing comment should be Code
        assert_eq!(
            classifier.classify_line("let x = 5; // comment", &mut state),
            LineType::Code
        );
    }

    #[test]
    fn test_classify_multi_line_comment_single_line() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Complete multi-line comment on one line
        assert_eq!(
            classifier.classify_line("/* comment */", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("  /* comment */  ", &mut state),
            LineType::Comment
        );
    }

    #[test]
    fn test_classify_multi_line_comment_with_code() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Multi-line comment with code after
        assert_eq!(
            classifier.classify_line("/* comment */ let x = 5;", &mut state),
            LineType::Code
        );

        // Code with multi-line comment after
        assert_eq!(
            classifier.classify_line("let x = 5; /* comment */", &mut state),
            LineType::Code
        );
    }

    #[test]
    fn test_classify_multi_line_comment_spanning() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Start of multi-line comment
        assert_eq!(
            classifier.classify_line("/* start of comment", &mut state),
            LineType::Comment
        );
        assert!(state.in_multi_line_comment);

        // Middle of multi-line comment
        assert_eq!(
            classifier.classify_line("still in comment", &mut state),
            LineType::Comment
        );
        assert!(state.in_multi_line_comment);

        // End of multi-line comment
        assert_eq!(
            classifier.classify_line("end of comment */", &mut state),
            LineType::Comment
        );
        assert!(!state.in_multi_line_comment);
    }

    #[test]
    fn test_classify_multi_line_comment_with_code_after_end() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(
            classifier.classify_line("/* comment", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("more comment */ let x = 5;", &mut state),
            LineType::Code
        );
        assert!(!state.in_multi_line_comment);
    }

    #[test]
    fn test_classify_shebang() {
        let lang = get_python_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(
            classifier.classify_line("#!/usr/bin/env python3", &mut state),
            LineType::Code
        );
    }

    #[test]
    fn test_classify_python_triple_quotes() {
        let lang = get_python_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Single line docstring
        assert_eq!(
            classifier.classify_line("\"\"\"This is a docstring\"\"\"", &mut state),
            LineType::Comment
        );

        // Multi-line docstring
        state.reset();
        assert_eq!(
            classifier.classify_line("\"\"\"", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("Docstring content", &mut state),
            LineType::Comment
        );
        assert_eq!(
            classifier.classify_line("\"\"\"", &mut state),
            LineType::Comment
        );
    }

    #[test]
    fn test_classify_file() {
        let lang = get_rust_language();
        let lines = vec![
            "// File header comment".to_string(),
            "".to_string(),
            "fn main() {".to_string(),
            "    let x = 5; // inline comment".to_string(),
            "    /* multi-line".to_string(),
            "       comment */".to_string(),
            "    println!(\"hello\");".to_string(),
            "}".to_string(),
        ];

        let stats = classify_file(&lines, lang);
        assert_eq!(stats.blank, 1);
        assert_eq!(stats.comment, 3); // header + 2 lines of multi-line comment
        assert_eq!(stats.code, 4); // main, let x (with comment), println, }
    }

    #[test]
    fn test_empty_multi_line_comment() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        assert_eq!(
            classifier.classify_line("/**/", &mut state),
            LineType::Comment
        );
    }

    #[test]
    fn test_code_before_multi_line_start() {
        let lang = get_rust_language();
        let classifier = LineClassifier::new(lang);
        let mut state = ClassifierState::new();

        // Code before multi-line comment start
        assert_eq!(
            classifier.classify_line("let x = 5; /* comment", &mut state),
            LineType::Code
        );
        assert!(state.in_multi_line_comment);

        assert_eq!(
            classifier.classify_line("continues */", &mut state),
            LineType::Comment
        );
    }
}
