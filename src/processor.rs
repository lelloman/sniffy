//! File processing.
//!
//! This module handles reading files, detecting binary files,
//! and coordinating line classification.

use crate::classifier::classify_file;
use crate::language::LanguageDetector;
use crate::stats::FileStats;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Check if a file is binary by looking for null bytes.
///
/// Reads the first 8KB of the file and checks for null bytes.
/// If any are found, the file is likely binary.
pub fn is_binary_file(path: &Path) -> std::io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 8192]; // 8KB buffer

    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        // Empty file, treat as text
        return Ok(false);
    }

    // Check for null bytes
    Ok(buffer[..bytes_read].contains(&0))
}

/// File processor that analyzes files and returns statistics.
pub struct FileProcessor {
    detector: LanguageDetector,
}

impl FileProcessor {
    /// Create a new FileProcessor.
    pub fn new() -> Self {
        Self {
            detector: LanguageDetector::new(),
        }
    }

    /// Process a file and return its language and statistics.
    ///
    /// Returns None if:
    /// - The file is binary
    /// - The language is not recognized
    /// - IO errors occur (logged but not returned)
    pub fn process_file(&self, path: &Path) -> Option<(String, FileStats)> {
        // Check if file is binary
        match is_binary_file(path) {
            Ok(true) => return None, // Binary file, skip
            Err(_) => return None,   // Error checking, skip
            Ok(false) => {}          // Text file, continue
        }

        // Detect language from file extension
        let language = self.detector.detect_from_path(path)?;

        // Open and read the file
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        // Read lines, handling UTF-8 errors gracefully
        for (line_num, line_result) in reader.lines().enumerate() {
            match line_result {
                Ok(line) => lines.push(line),
                Err(_) => {
                    // Skip invalid UTF-8 lines
                    // In a real implementation, we might log this
                    eprintln!(
                        "Warning: Skipping line {} in {} due to encoding error",
                        line_num + 1,
                        path.display()
                    );
                }
            }
        }

        // Classify the file
        let stats = classify_file(&lines, language);

        Some((language.name.to_string(), stats))
    }
}

impl Default for FileProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_binary_text_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "This is a text file\nwith multiple lines\n").unwrap();
        temp_file.flush().unwrap();

        let is_binary = is_binary_file(temp_file.path()).unwrap();
        assert!(!is_binary);
    }

    #[test]
    fn test_is_binary_binary_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write some binary data with null bytes
        let binary_data = vec![0x89, 0x50, 0x4E, 0x47, 0x00, 0x0D, 0x0A, 0x1A];
        temp_file.write_all(&binary_data).unwrap();
        temp_file.flush().unwrap();

        let is_binary = is_binary_file(temp_file.path()).unwrap();
        assert!(is_binary);
    }

    #[test]
    fn test_is_binary_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let is_binary = is_binary_file(temp_file.path()).unwrap();
        assert!(!is_binary);
    }

    #[test]
    fn test_file_processor_new() {
        let _processor = FileProcessor::new();
        // Just verify it creates successfully
    }

    #[test]
    fn test_process_rust_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().with_extension("rs");

        // Write a simple Rust file
        writeln!(temp_file, "// This is a comment").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "fn main() {{").unwrap();
        writeln!(temp_file, "    let x = 5;").unwrap();
        writeln!(temp_file, "}}").unwrap();
        temp_file.flush().unwrap();

        // Copy to a .rs file
        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let processor = FileProcessor::new();
        let result = processor.process_file(&temp_path);

        assert!(result.is_some());
        let (language, stats) = result.unwrap();
        assert_eq!(language, "Rust");
        assert_eq!(stats.blank, 1);
        assert_eq!(stats.comment, 1);
        assert_eq!(stats.code, 3);

        // Cleanup
        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_process_binary_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().with_extension("bin");

        // Write binary data
        let binary_data = vec![0x89, 0x50, 0x4E, 0x47, 0x00, 0x0D, 0x0A, 0x1A];
        temp_file.write_all(&binary_data).unwrap();
        temp_file.flush().unwrap();

        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let processor = FileProcessor::new();
        let result = processor.process_file(&temp_path);

        // Should return None for binary files
        assert!(result.is_none());

        // Cleanup
        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_process_unknown_extension() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().with_extension("unknown");

        writeln!(temp_file, "some content").unwrap();
        temp_file.flush().unwrap();

        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let processor = FileProcessor::new();
        let result = processor.process_file(&temp_path);

        // Should return None for unknown extensions
        assert!(result.is_none());

        // Cleanup
        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_process_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().with_extension("rs");

        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let processor = FileProcessor::new();
        let result = processor.process_file(&temp_path);

        assert!(result.is_some());
        let (language, stats) = result.unwrap();
        assert_eq!(language, "Rust");
        assert_eq!(stats.blank, 0);
        assert_eq!(stats.comment, 0);
        assert_eq!(stats.code, 0);

        // Cleanup
        std::fs::remove_file(&temp_path).ok();
    }
}
