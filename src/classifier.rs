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

#[cfg(test)]
mod tests {
    use super::*;

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
}
