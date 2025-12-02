//! Directory walking and file discovery.
//!
//! This module implements recursive directory traversal,
//! respecting .gitignore patterns and skip rules.

use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Directory walker that respects .gitignore and other ignore files.
pub struct DirectoryWalker {
    paths: Vec<PathBuf>,
    hidden: bool,
}

impl DirectoryWalker {
    /// Create a new DirectoryWalker for the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            paths: vec![path.as_ref().to_path_buf()],
            hidden: false,
        }
    }

    /// Set whether to include hidden files and directories.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Walk the directory and yield all file paths.
    pub fn walk(&self) -> impl Iterator<Item = PathBuf> {
        let mut builder = WalkBuilder::new(&self.paths[0]);

        // Configure walker
        builder.hidden(!self.hidden);
        builder.git_ignore(true);
        builder.git_global(true);
        builder.git_exclude(true);

        // Add additional paths if any
        for path in &self.paths[1..] {
            builder.add(path);
        }

        builder
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.into_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_directory_walker_new() {
        let temp_dir = TempDir::new().unwrap();
        let walker = DirectoryWalker::new(temp_dir.path());
        assert!(!walker.hidden);
    }

    #[test]
    fn test_directory_walker_hidden() {
        let temp_dir = TempDir::new().unwrap();
        let walker = DirectoryWalker::new(temp_dir.path()).hidden(true);
        assert!(walker.hidden);
    }

    #[test]
    fn test_walk_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create some test files
        let file1 = temp_dir.path().join("test1.rs");
        let file2 = temp_dir.path().join("test2.rs");
        fs::File::create(&file1).unwrap();
        fs::File::create(&file2).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_walk_respects_gitignore() {
        let temp_dir = TempDir::new().unwrap();

        // Initialize git repository (required for .gitignore to work)
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to initialize git repo");

        // Create .gitignore and flush it
        let gitignore_path = temp_dir.path().join(".gitignore");
        let mut gitignore = fs::File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "ignored.rs").unwrap();
        gitignore.sync_all().unwrap();
        drop(gitignore); // Ensure file is closed

        // Create files
        let included = temp_dir.path().join("included.rs");
        let ignored = temp_dir.path().join("ignored.rs");
        fs::File::create(&included).unwrap();
        fs::File::create(&ignored).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Filter out .gitignore file and check for ignored.rs
        let has_included = files.iter().any(|p| p.ends_with("included.rs"));
        let has_ignored = files.iter().any(|p| p.ends_with("ignored.rs"));

        assert!(has_included, "included.rs should be present");
        assert!(!has_ignored, "ignored.rs should be excluded");
    }

    #[test]
    fn test_walk_excludes_hidden_by_default() {
        let temp_dir = TempDir::new().unwrap();

        // Create regular and hidden files
        let regular = temp_dir.path().join("regular.rs");
        let hidden = temp_dir.path().join(".hidden.rs");
        fs::File::create(&regular).unwrap();
        fs::File::create(&hidden).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should exclude hidden files by default
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("regular.rs"));
    }

    #[test]
    fn test_walk_includes_hidden_when_enabled() {
        let temp_dir = TempDir::new().unwrap();

        // Create regular and hidden files
        let regular = temp_dir.path().join("regular.rs");
        let hidden = temp_dir.path().join(".hidden.rs");
        fs::File::create(&regular).unwrap();
        fs::File::create(&hidden).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path()).hidden(true);
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should include both files
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_walk_excludes_directories() {
        let temp_dir = TempDir::new().unwrap();

        // Create a subdirectory with a file
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file_in_subdir = subdir.join("test.rs");
        fs::File::create(&file_in_subdir).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only return files, not directories
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("test.rs"));
    }
}
