//! Error types and handling.
//!
//! This module defines custom error types for Sniffy
//! and conversions from standard error types.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Custom error type for Sniffy operations.
#[derive(Debug)]
pub enum SniffyError {
    /// IO error occurred during file operations.
    Io(io::Error),
    /// Invalid path provided.
    InvalidPath(PathBuf),
    /// File encoding error (not valid UTF-8).
    EncodingError { path: PathBuf, line: usize },
    /// File processing error.
    ProcessingError { path: PathBuf, message: String },
}

impl fmt::Display for SniffyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SniffyError::Io(err) => write!(f, "IO error: {}", err),
            SniffyError::InvalidPath(path) => write!(f, "Invalid path: {}", path.display()),
            SniffyError::EncodingError { path, line } => {
                write!(
                    f,
                    "Encoding error in file {} at line {}",
                    path.display(),
                    line
                )
            }
            SniffyError::ProcessingError { path, message } => {
                write!(f, "Error processing {}: {}", path.display(), message)
            }
        }
    }
}

impl std::error::Error for SniffyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SniffyError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for SniffyError {
    fn from(err: io::Error) -> Self {
        SniffyError::Io(err)
    }
}

/// Type alias for Result with SniffyError.
pub type Result<T> = std::result::Result<T, SniffyError>;
