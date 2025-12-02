//! File processing.
//!
//! This module handles reading files, detecting binary files,
//! and coordinating line classification.

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
}
