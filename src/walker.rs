//! Directory walking and file discovery.
//!
//! This module implements recursive directory traversal,
//! respecting .gitignore patterns and skip rules.

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Directory walker that respects .gitignore and other ignore files.
pub struct DirectoryWalker {
    paths: Vec<PathBuf>,
    hidden: bool,
    exclude: Vec<String>,
    include: Vec<String>,
}

/// Check if a file should be skipped based on common patterns.
///
/// This function filters out:
/// - Common dependency directories (node_modules, target, etc.)
/// - Build output directories (build, dist, etc.)
/// - Version control directories (.git, .svn, etc.)
/// - Python cache directories (__pycache__, .venv, venv)
/// - Minified files (*.min.js, *.min.css)
/// - Lock files (package-lock.json, Cargo.lock, etc.)
fn should_skip_file(path: &Path) -> bool {
    // Skip directories by checking path components
    for component in path.components() {
        if let Some(component_str) = component.as_os_str().to_str() {
            match component_str {
                // Dependency directories
                "node_modules" | "vendor" | "bower_components" => return true,
                // Build output directories
                "target" | "build" | "dist" | "out" | ".next" => return true,
                // Version control directories
                ".git" | ".svn" | ".hg" => return true,
                // Python virtual environments and cache
                ".venv" | "venv" | "__pycache__" | ".pytest_cache" => return true,
                // IDE and editor directories
                ".idea" | ".vscode" | ".vs" => return true,
                // OS-specific directories
                ".DS_Store" => return true,
                _ => {}
            }
        }
    }

    // Skip specific file patterns
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        // Minified files
        if file_name.ends_with(".min.js") || file_name.ends_with(".min.css") {
            return true;
        }

        // Lock files
        match file_name {
            "package-lock.json" | "yarn.lock" | "pnpm-lock.yaml" | "Cargo.lock"
            | "Gemfile.lock" | "poetry.lock" | "composer.lock" | "go.sum" => return true,
            _ => {}
        }
    }

    false
}

impl DirectoryWalker {
    /// Create a new DirectoryWalker for the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            paths: vec![path.as_ref().to_path_buf()],
            hidden: false,
            exclude: Vec::new(),
            include: Vec::new(),
        }
    }

    /// Set whether to include hidden files and directories.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Set exclude patterns (glob patterns to exclude).
    pub fn exclude(mut self, patterns: Vec<String>) -> Self {
        self.exclude = patterns;
        self
    }

    /// Set include patterns (glob patterns to include, overrides excludes).
    pub fn include(mut self, patterns: Vec<String>) -> Self {
        self.include = patterns;
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

        // Build overrides for include/exclude patterns
        let mut override_builder = OverrideBuilder::new(&self.paths[0]);

        // Add exclude patterns (as negative globs)
        for pattern in &self.exclude {
            // The ignore crate uses ! prefix for negation (include),
            // so exclude patterns are added as-is
            let _ = override_builder.add(&format!("!{}", pattern));
        }

        // Add include patterns (these take precedence)
        // When include patterns are specified, we want to only match those patterns
        for pattern in &self.include {
            let _ = override_builder.add(pattern);
        }

        if let Ok(overrides) = override_builder.build() {
            builder.overrides(overrides);
        }

        builder
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.into_path())
            .filter(|path| !should_skip_file(path))
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

    #[test]
    fn test_skip_node_modules() {
        let temp_dir = TempDir::new().unwrap();

        // Create node_modules directory with a file
        let node_modules = temp_dir.path().join("node_modules");
        fs::create_dir(&node_modules).unwrap();
        let file_in_node_modules = node_modules.join("package.js");
        fs::File::create(&file_in_node_modules).unwrap();

        // Create a regular file
        let regular_file = temp_dir.path().join("app.js");
        fs::File::create(&regular_file).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find the regular file, not the one in node_modules
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("app.js"));
    }

    #[test]
    fn test_skip_target_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create target directory with a file
        let target = temp_dir.path().join("target");
        fs::create_dir(&target).unwrap();
        let file_in_target = target.join("binary");
        fs::File::create(&file_in_target).unwrap();

        // Create a regular file
        let regular_file = temp_dir.path().join("main.rs");
        fs::File::create(&regular_file).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find the regular file
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.rs"));
    }

    #[test]
    fn test_skip_build_directories() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple build directories
        let build = temp_dir.path().join("build");
        let dist = temp_dir.path().join("dist");
        fs::create_dir(&build).unwrap();
        fs::create_dir(&dist).unwrap();

        fs::File::create(build.join("output.js")).unwrap();
        fs::File::create(dist.join("bundle.js")).unwrap();

        // Create a regular file
        let regular_file = temp_dir.path().join("source.js");
        fs::File::create(&regular_file).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find the regular file
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("source.js"));
    }

    #[test]
    fn test_skip_minified_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create minified files
        fs::File::create(temp_dir.path().join("app.min.js")).unwrap();
        fs::File::create(temp_dir.path().join("style.min.css")).unwrap();

        // Create regular files
        fs::File::create(temp_dir.path().join("app.js")).unwrap();
        fs::File::create(temp_dir.path().join("style.css")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find non-minified files
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("app.js")));
        assert!(files.iter().any(|p| p.ends_with("style.css")));
        assert!(!files.iter().any(|p| p.ends_with("app.min.js")));
        assert!(!files.iter().any(|p| p.ends_with("style.min.css")));
    }

    #[test]
    fn test_skip_lock_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create various lock files
        fs::File::create(temp_dir.path().join("package-lock.json")).unwrap();
        fs::File::create(temp_dir.path().join("Cargo.lock")).unwrap();
        fs::File::create(temp_dir.path().join("yarn.lock")).unwrap();
        fs::File::create(temp_dir.path().join("Gemfile.lock")).unwrap();

        // Create regular files
        fs::File::create(temp_dir.path().join("package.json")).unwrap();
        fs::File::create(temp_dir.path().join("Cargo.toml")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find non-lock files
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("package.json")));
        assert!(files.iter().any(|p| p.ends_with("Cargo.toml")));
        assert!(!files
            .iter()
            .any(|p| p.file_name().unwrap().to_str().unwrap().contains("lock")));
    }

    #[test]
    fn test_skip_python_venv() {
        let temp_dir = TempDir::new().unwrap();

        // Create virtual environment directories
        let venv = temp_dir.path().join("venv");
        let dot_venv = temp_dir.path().join(".venv");
        let pycache = temp_dir.path().join("__pycache__");

        fs::create_dir(&venv).unwrap();
        fs::create_dir(&dot_venv).unwrap();
        fs::create_dir(&pycache).unwrap();

        fs::File::create(venv.join("activate")).unwrap();
        fs::File::create(dot_venv.join("lib.py")).unwrap();
        fs::File::create(pycache.join("module.pyc")).unwrap();

        // Create regular Python file
        fs::File::create(temp_dir.path().join("main.py")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find the regular Python file
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.py"));
    }

    #[test]
    fn test_skip_ide_directories() {
        let temp_dir = TempDir::new().unwrap();

        // Create IDE directories
        let vscode = temp_dir.path().join(".vscode");
        let idea = temp_dir.path().join(".idea");

        fs::create_dir(&vscode).unwrap();
        fs::create_dir(&idea).unwrap();

        fs::File::create(vscode.join("settings.json")).unwrap();
        fs::File::create(idea.join("workspace.xml")).unwrap();

        // Create regular file
        fs::File::create(temp_dir.path().join("code.rs")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path());
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find the regular file
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("code.rs"));
    }

    #[test]
    fn test_should_skip_file_function() {
        // Test dependency directories
        assert!(should_skip_file(Path::new(
            "/project/node_modules/package/index.js"
        )));
        assert!(should_skip_file(Path::new("/project/vendor/lib.rb")));

        // Test build directories
        assert!(should_skip_file(Path::new(
            "/project/target/release/binary"
        )));
        assert!(should_skip_file(Path::new("/project/build/output.js")));
        assert!(should_skip_file(Path::new("/project/dist/bundle.js")));

        // Test version control
        assert!(should_skip_file(Path::new("/project/.git/config")));
        assert!(should_skip_file(Path::new("/project/.svn/entries")));

        // Test Python
        assert!(should_skip_file(Path::new("/project/venv/lib/python.py")));
        assert!(should_skip_file(Path::new("/project/.venv/activate")));
        assert!(should_skip_file(Path::new(
            "/project/__pycache__/module.pyc"
        )));

        // Test minified files
        assert!(should_skip_file(Path::new("/project/app.min.js")));
        assert!(should_skip_file(Path::new("/project/style.min.css")));

        // Test lock files
        assert!(should_skip_file(Path::new("/project/package-lock.json")));
        assert!(should_skip_file(Path::new("/project/Cargo.lock")));
        assert!(should_skip_file(Path::new("/project/yarn.lock")));

        // Test that regular files are NOT skipped
        assert!(!should_skip_file(Path::new("/project/src/main.rs")));
        assert!(!should_skip_file(Path::new("/project/app.js")));
        assert!(!should_skip_file(Path::new("/project/style.css")));
        assert!(!should_skip_file(Path::new("/project/Cargo.toml")));
    }

    #[test]
    fn test_exclude_pattern() {
        let temp_dir = TempDir::new().unwrap();

        // Create various files
        fs::File::create(temp_dir.path().join("main.rs")).unwrap();
        fs::File::create(temp_dir.path().join("lib.rs")).unwrap();
        fs::File::create(temp_dir.path().join("test.js")).unwrap();
        fs::File::create(temp_dir.path().join("app.js")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path()).exclude(vec!["*.js".to_string()]);
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find .rs files
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("main.rs")));
        assert!(files.iter().any(|p| p.ends_with("lib.rs")));
        assert!(!files.iter().any(|p| p.ends_with(".js")));
    }

    #[test]
    fn test_include_pattern() {
        let temp_dir = TempDir::new().unwrap();

        // Create various files
        fs::File::create(temp_dir.path().join("main.rs")).unwrap();
        fs::File::create(temp_dir.path().join("lib.rs")).unwrap();
        fs::File::create(temp_dir.path().join("test.js")).unwrap();
        fs::File::create(temp_dir.path().join("app.py")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path()).include(vec!["*.rs".to_string()]);
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find .rs files
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("main.rs")));
        assert!(files.iter().any(|p| p.ends_with("lib.rs")));
    }

    #[test]
    fn test_multiple_include_patterns() {
        let temp_dir = TempDir::new().unwrap();

        // Create various files
        fs::File::create(temp_dir.path().join("main.rs")).unwrap();
        fs::File::create(temp_dir.path().join("test.js")).unwrap();
        fs::File::create(temp_dir.path().join("app.py")).unwrap();
        fs::File::create(temp_dir.path().join("style.css")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path())
            .include(vec!["*.rs".to_string(), "*.js".to_string()]);
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should find .rs and .js files
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("main.rs")));
        assert!(files.iter().any(|p| p.ends_with("test.js")));
    }

    #[test]
    fn test_exclude_directory_pattern() {
        let temp_dir = TempDir::new().unwrap();

        // Create directories
        let src_dir = temp_dir.path().join("src");
        let tests_dir = temp_dir.path().join("tests");
        fs::create_dir(&src_dir).unwrap();
        fs::create_dir(&tests_dir).unwrap();

        // Create files
        fs::File::create(src_dir.join("main.rs")).unwrap();
        fs::File::create(tests_dir.join("test_main.rs")).unwrap();

        let walker = DirectoryWalker::new(temp_dir.path()).exclude(vec!["tests/**".to_string()]);
        let files: Vec<PathBuf> = walker.walk().collect();

        // Should only find files in src
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.rs"));
    }
}
