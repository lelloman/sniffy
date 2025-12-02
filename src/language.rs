//! Language definitions and detection.
//!
//! This module defines programming language information including
//! file extensions and comment syntax for line classification.

use std::collections::HashMap;
use std::path::Path;

/// Represents a pair of multi-line comment delimiters.
#[derive(Debug, Clone)]
pub struct CommentPair {
    pub start: &'static str,
    pub end: &'static str,
}

impl CommentPair {
    pub const fn new(start: &'static str, end: &'static str) -> Self {
        Self { start, end }
    }
}

/// Information about a programming language.
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub single_line_comments: &'static [&'static str],
    pub multi_line_comments: &'static [CommentPair],
}

impl LanguageInfo {
    pub const fn new(
        name: &'static str,
        extensions: &'static [&'static str],
        single_line_comments: &'static [&'static str],
        multi_line_comments: &'static [CommentPair],
    ) -> Self {
        Self {
            name,
            extensions,
            single_line_comments,
            multi_line_comments,
        }
    }
}

/// Static array of all supported languages.
pub const LANGUAGES: &[LanguageInfo] = &[
    // JavaScript
    LanguageInfo::new(
        "JavaScript",
        &["js", "jsx", "mjs", "cjs"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // TypeScript
    LanguageInfo::new(
        "TypeScript",
        &["ts", "tsx"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Python
    LanguageInfo::new(
        "Python",
        &["py", "pyw"],
        &["#"],
        &[CommentPair::new("\"\"\"", "\"\"\""), CommentPair::new("'''", "'''")],
    ),
    // Rust
    LanguageInfo::new(
        "Rust",
        &["rs"],
        &["//", "///", "//!"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Go
    LanguageInfo::new(
        "Go",
        &["go"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Java
    LanguageInfo::new(
        "Java",
        &["java"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // C
    LanguageInfo::new(
        "C",
        &["c", "h"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // C++
    LanguageInfo::new(
        "C++",
        &["cpp", "cc", "cxx", "hpp", "hxx", "hh"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // C#
    LanguageInfo::new(
        "C#",
        &["cs"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Ruby
    LanguageInfo::new(
        "Ruby",
        &["rb"],
        &["#"],
        &[CommentPair::new("=begin", "=end")],
    ),
    // Shell
    LanguageInfo::new(
        "Shell",
        &["sh", "bash", "zsh"],
        &["#"],
        &[],
    ),
    // HTML
    LanguageInfo::new(
        "HTML",
        &["html", "htm"],
        &[],
        &[CommentPair::new("<!--", "-->")],
    ),
    // CSS
    LanguageInfo::new(
        "CSS",
        &["css"],
        &[],
        &[CommentPair::new("/*", "*/")],
    ),
    // SCSS
    LanguageInfo::new(
        "SCSS",
        &["scss"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Sass
    LanguageInfo::new(
        "Sass",
        &["sass"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Markdown
    LanguageInfo::new(
        "Markdown",
        &["md", "markdown"],
        &[],
        &[CommentPair::new("<!--", "-->")],
    ),
    // JSON
    LanguageInfo::new(
        "JSON",
        &["json"],
        &[],
        &[],
    ),
    // YAML
    LanguageInfo::new(
        "YAML",
        &["yaml", "yml"],
        &["#"],
        &[],
    ),
    // XML
    LanguageInfo::new(
        "XML",
        &["xml"],
        &[],
        &[CommentPair::new("<!--", "-->")],
    ),
    // PHP
    LanguageInfo::new(
        "PHP",
        &["php"],
        &["//", "#"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Kotlin
    LanguageInfo::new(
        "Kotlin",
        &["kt", "kts"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Swift
    LanguageInfo::new(
        "Swift",
        &["swift"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Scala
    LanguageInfo::new(
        "Scala",
        &["scala"],
        &["//"],
        &[CommentPair::new("/*", "*/")],
    ),
    // Elixir
    LanguageInfo::new(
        "Elixir",
        &["ex", "exs"],
        &["#"],
        &[],
    ),
    // Erlang
    LanguageInfo::new(
        "Erlang",
        &["erl", "hrl"],
        &["%"],
        &[],
    ),
    // Haskell
    LanguageInfo::new(
        "Haskell",
        &["hs", "lhs"],
        &["--"],
        &[CommentPair::new("{-", "-}")],
    ),
    // Lua
    LanguageInfo::new(
        "Lua",
        &["lua"],
        &["--"],
        &[CommentPair::new("--[[", "]]")],
    ),
    // Perl
    LanguageInfo::new(
        "Perl",
        &["pl", "pm"],
        &["#"],
        &[CommentPair::new("=pod", "=cut")],
    ),
    // R
    LanguageInfo::new(
        "R",
        &["r", "R"],
        &["#"],
        &[],
    ),
    // SQL
    LanguageInfo::new(
        "SQL",
        &["sql"],
        &["--"],
        &[CommentPair::new("/*", "*/")],
    ),
    // TOML
    LanguageInfo::new(
        "TOML",
        &["toml"],
        &["#"],
        &[],
    ),
    // INI
    LanguageInfo::new(
        "INI",
        &["ini", "cfg"],
        &[";", "#"],
        &[],
    ),
    // Vim Script
    LanguageInfo::new(
        "Vim Script",
        &["vim"],
        &["\""],
        &[],
    ),
];

/// Language detector that maps file extensions to languages.
pub struct LanguageDetector {
    extension_map: HashMap<String, &'static LanguageInfo>,
}

impl LanguageDetector {
    /// Create a new LanguageDetector with all supported languages.
    pub fn new() -> Self {
        let mut extension_map = HashMap::new();

        for lang in LANGUAGES {
            for ext in lang.extensions {
                extension_map.insert(ext.to_string(), lang);
            }
        }

        Self { extension_map }
    }

    /// Detect the language of a file based on its path.
    ///
    /// Returns None if the extension is not recognized.
    pub fn detect_from_path(&self, path: &Path) -> Option<&'static LanguageInfo> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| self.extension_map.get(&ext.to_lowercase()))
            .copied()
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_language_detector_new() {
        let detector = LanguageDetector::new();
        assert!(!detector.extension_map.is_empty());
    }

    #[test]
    fn test_detect_rust() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("main.rs");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Rust");
    }

    #[test]
    fn test_detect_python() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("script.py");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Python");
    }

    #[test]
    fn test_detect_javascript() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("app.js");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "JavaScript");
    }

    #[test]
    fn test_detect_typescript() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("component.tsx");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "TypeScript");
    }

    #[test]
    fn test_detect_case_insensitive() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("main.RS");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Rust");
    }

    #[test]
    fn test_detect_unknown_extension() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("file.unknown");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_none());
    }

    #[test]
    fn test_detect_no_extension() {
        let detector = LanguageDetector::new();
        let path = PathBuf::from("Makefile");
        let lang = detector.detect_from_path(&path);
        assert!(lang.is_none());
    }

    #[test]
    fn test_c_vs_cpp() {
        let detector = LanguageDetector::new();

        let c_path = PathBuf::from("main.c");
        let c_lang = detector.detect_from_path(&c_path);
        assert_eq!(c_lang.unwrap().name, "C");

        let cpp_path = PathBuf::from("main.cpp");
        let cpp_lang = detector.detect_from_path(&cpp_path);
        assert_eq!(cpp_lang.unwrap().name, "C++");
    }

    #[test]
    fn test_all_languages_have_extensions() {
        for lang in LANGUAGES {
            assert!(!lang.extensions.is_empty(), "Language {} has no extensions", lang.name);
        }
    }
}
