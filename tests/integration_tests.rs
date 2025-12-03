use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to get the path to test fixtures
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_simple_rust_file() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple/main.rs"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("1")); // 1 file
}

#[test]
fn test_multi_lang_directory() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("multi_lang"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("JavaScript"))
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Python"));
}

#[test]
fn test_simple_directory_totals() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Python"));
}

#[test]
fn test_binary_file_skipped() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("edge_cases"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_json_output_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple/main.rs"))
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"language\""))
        .stdout(predicate::str::contains("\"Rust\""))
        .stdout(predicate::str::contains("\"blank\""))
        .stdout(predicate::str::contains("\"comment\""))
        .stdout(predicate::str::contains("\"code\""));
}

#[test]
fn test_csv_output_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple/main.rs"))
        .arg("--format")
        .arg("csv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "language,files,blank,comment,code,total",
        ))
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_hidden_files_excluded_by_default() {
    let temp_dir = TempDir::new().unwrap();

    // Create regular and hidden files
    fs::write(temp_dir.path().join("regular.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join(".hidden.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("1")); // Should show 1 file
}

#[test]
fn test_hidden_files_included_with_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Create regular and hidden files
    fs::write(temp_dir.path().join("regular.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join(".hidden.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path()).arg("--hidden");

    cmd.assert().success().stdout(predicate::str::contains("2"));
}

#[test]
fn test_invalid_path_error() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg("/nonexistent/path/that/does/not/exist");

    cmd.assert().failure().stderr(
        predicate::str::contains("does not exist").or(predicate::str::contains("not found")),
    );
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    // Should succeed but report no files
    cmd.assert().success();
}

#[test]
fn test_verbose_mode() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple")).arg("--verbose");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Processing").or(predicate::str::contains("files")));
}

#[test]
fn test_parallel_jobs_option() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple")).arg("--jobs").arg("2");

    cmd.assert().success();
}

#[test]
fn test_node_modules_skipped() {
    let temp_dir = TempDir::new().unwrap();

    // Create node_modules directory with a file
    let node_modules = temp_dir.path().join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    fs::write(node_modules.join("package.js"), "module.exports = {};").unwrap();

    // Create regular file
    fs::write(temp_dir.path().join("app.js"), "console.log('hi');").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("1"));
}

#[test]
fn test_minified_files_skipped() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("app.js"), "console.log('hi');").unwrap();
    fs::write(
        temp_dir.path().join("app.min.js"),
        "console.log('minified');",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("1"));
}

#[test]
fn test_lock_files_skipped() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("package.json"), "{}").unwrap();
    fs::write(temp_dir.path().join("package-lock.json"), "{}").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("1"));
}

#[test]
#[cfg(unix)] // Permission tests are Unix-specific
fn test_permission_denied_directory() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();

    // Create a subdirectory with a file
    let forbidden_dir = temp_dir.path().join("forbidden");
    fs::create_dir(&forbidden_dir).unwrap();
    fs::write(forbidden_dir.join("secret.rs"), "fn secret() {}").unwrap();

    // Create a regular accessible file
    fs::write(temp_dir.path().join("public.rs"), "fn public() {}").unwrap();

    // Remove read and execute permissions from the forbidden directory
    let mut perms = fs::metadata(&forbidden_dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&forbidden_dir, perms).unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    // Should succeed but skip the forbidden directory
    let output = cmd.assert().success();

    // Should only count the public file
    output.stdout(predicate::str::contains("1"));

    // Restore permissions for cleanup
    let mut perms = fs::metadata(&forbidden_dir).unwrap().permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(&forbidden_dir, perms); // Ignore errors during cleanup
}

#[test]
fn test_multiple_paths() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();

    fs::write(temp_dir1.path().join("file1.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir2.path().join("file2.py"), "def main(): pass").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir1.path()).arg(temp_dir2.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Python"))
        .stdout(predicate::str::contains("2")); // 2 total files
}

#[test]
fn test_exact_counts_simple_rust() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple/main.rs"))
        .arg("--format")
        .arg("json");

    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Verify counts (note: fixture has comment explaining counts, which itself gets counted)
    assert!(stdout.contains("\"blank\":4") || stdout.contains("\"blank\": 4"));
    assert!(stdout.contains("\"comment\":7") || stdout.contains("\"comment\": 7"));
    assert!(stdout.contains("\"code\":6") || stdout.contains("\"code\": 6"));
}

#[test]
fn test_exact_counts_multi_lang() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("multi_lang"))
        .arg("--format")
        .arg("csv");

    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Should have 3 files total
    assert!(stdout.contains("Total,3,"));
}

#[test]
fn test_parallel_jobs_zero() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple")).arg("--jobs").arg("0"); // Auto-detect CPUs

    cmd.assert().success();
}

#[test]
fn test_parallel_jobs_one() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple")).arg("--jobs").arg("1"); // Single-threaded

    cmd.assert().success();
}

#[test]
fn test_git_history_on_git_repo() {
    // Test on the sniffy repo itself
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".").arg("--history");

    let output = cmd.assert().success();

    // Should contain history-related output
    output.stdout(predicate::str::contains("Git History").or(predicate::str::contains("Commits")));
}

#[test]
fn test_git_history_with_since() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--since")
        .arg("2025-01-01");

    cmd.assert().success();
}

#[test]
fn test_git_history_by_week() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".").arg("--history").arg("--by-week");

    cmd.assert().success();
}

#[test]
fn test_git_history_json_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".").arg("--history").arg("--format").arg("json");

    let output = cmd.assert().success();
    output.stdout(
        predicate::str::contains("\"time_series\"")
            .or(predicate::str::contains("\"total_commits\"")),
    );
}

#[test]
fn test_git_history_csv_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".").arg("--history").arg("--format").arg("csv");

    let output = cmd.assert().success();
    output.stdout(predicate::str::contains("date,").or(predicate::str::contains("week,")));
}

#[test]
fn test_binary_file_actually_skipped() {
    let temp_dir = TempDir::new().unwrap();

    // Create a text file
    fs::write(temp_dir.path().join("text.rs"), "fn main() {}").unwrap();

    // Create a binary file with null bytes
    let binary_path = temp_dir.path().join("binary.bin");
    fs::write(&binary_path, b"\x00\x01\x02\x03\x04\x05").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("1")); // Only 1 file (the .rs file)
}

#[test]
fn test_edge_case_only_comments() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("edge_cases/only_comments.rs"))
        .arg("--format")
        .arg("json");

    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Should have 0 code lines
    assert!(stdout.contains("\"code\":0") || stdout.contains("\"code\": 0"));
}

#[test]
fn test_edge_case_only_code() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("edge_cases/only_code.rs"))
        .arg("--format")
        .arg("json");

    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Should have 0 comments
    assert!(stdout.contains("\"comment\":0") || stdout.contains("\"comment\": 0"));
}

#[test]
fn test_verbose_shows_file_count() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple")).arg("--verbose");

    let output = cmd.assert().success();
    output.stderr(predicate::str::contains("files"));
}

#[test]
fn test_table_format_explicit() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(fixture_path("simple/main.rs"))
        .arg("--format")
        .arg("table");

    let output = cmd.assert().success();
    // Table format should have box drawing characters
    output.stdout(predicate::str::contains("│"));
}

#[test]
fn test_nested_directories() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested structure
    let nested = temp_dir.path().join("src").join("utils");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("helper.rs"), "fn help() {}").unwrap();
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert().success().stdout(predicate::str::contains("2")); // 2 files
}

// New Phase 11 integration tests

#[test]
fn test_git_history_with_until() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--since")
        .arg("2025-12-01")
        .arg("--until")
        .arg("2025-12-31");

    cmd.assert().success();
}

#[test]
fn test_git_history_with_last_days() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".").arg("--history").arg("--last").arg("30");

    let output = cmd.assert().success();
    output.stdout(predicate::str::contains("Git History"));
}

#[test]
fn test_git_history_verbose_shows_progress() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--last")
        .arg("7")
        .arg("--verbose");

    let output = cmd.assert().success();
    // Verbose mode should show completion message on stderr
    output.stderr(predicate::str::contains("Completed analyzing"));
}

#[test]
fn test_git_history_until_without_since() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--until")
        .arg("2025-12-01");

    cmd.assert().success();
}

#[test]
fn test_git_history_date_range_json() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--since")
        .arg("2025-12-01")
        .arg("--until")
        .arg("2025-12-31")
        .arg("--format")
        .arg("json");

    let output = cmd.assert().success();
    output.stdout(predicate::str::contains("\"total_commits\""));
}

#[test]
fn test_git_history_last_with_csv() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--last")
        .arg("14")
        .arg("--format")
        .arg("csv");

    let output = cmd.assert().success();
    output.stdout(predicate::str::contains("date,"));
}

#[test]
fn test_git_history_last_conflicts_with_since() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--last")
        .arg("7")
        .arg("--since")
        .arg("2025-12-01");

    // Should fail because --last conflicts with --since
    cmd.assert().failure();
}

#[test]
fn test_git_history_last_conflicts_with_until() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--last")
        .arg("7")
        .arg("--until")
        .arg("2025-12-31");

    // Should fail because --last conflicts with --until
    cmd.assert().failure();
}

#[test]
fn test_git_history_invalid_date_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--since")
        .arg("12/01/2025"); // Wrong format (MM/DD/YYYY)

    let output = cmd.assert().failure();
    output.stderr(predicate::str::contains("Invalid date format"));
}

#[test]
fn test_git_history_since_rfc3339_format() {
    let mut cmd = Command::cargo_bin("sniffy").unwrap();
    cmd.arg(".")
        .arg("--history")
        .arg("--since")
        .arg("2025-12-01T00:00:00Z");

    cmd.assert().success();
}
