use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sniffy::classifier::{classify_file, ClassifierState, LineClassifier};
use sniffy::language::{LanguageDetector, LANGUAGES};
use sniffy::processor::FileProcessor;
use sniffy::stats::{FileStats, ProjectStats};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

fn bench_line_classification(c: &mut Criterion) {
    let rust_lang = LANGUAGES.iter().find(|l| l.name == "Rust").unwrap();
    let classifier = LineClassifier::new(rust_lang);
    let mut state = ClassifierState::new();

    let mut group = c.benchmark_group("line_classification");

    // Benchmark blank line
    group.bench_function("blank_line", |b| {
        b.iter(|| {
            let mut state = ClassifierState::new();
            classifier.classify_line(black_box("   "), &mut state)
        })
    });

    // Benchmark single-line comment
    group.bench_function("single_comment", |b| {
        b.iter(|| {
            let mut state = ClassifierState::new();
            classifier.classify_line(black_box("// This is a comment"), &mut state)
        })
    });

    // Benchmark code line
    group.bench_function("code_line", |b| {
        b.iter(|| {
            let mut state = ClassifierState::new();
            classifier.classify_line(black_box("let x = 5;"), &mut state)
        })
    });

    // Benchmark mixed line (code + comment)
    group.bench_function("mixed_line", |b| {
        b.iter(|| {
            let mut state = ClassifierState::new();
            classifier.classify_line(black_box("let x = 5; // comment"), &mut state)
        })
    });

    group.finish();
}

fn bench_file_classification(c: &mut Criterion) {
    let rust_lang = LANGUAGES.iter().find(|l| l.name == "Rust").unwrap();

    let mut group = c.benchmark_group("file_classification");

    // Small file (10 lines)
    let small_file: Vec<String> = vec![
        "// File header".to_string(),
        "".to_string(),
        "fn main() {".to_string(),
        "    let x = 5;".to_string(),
        "    println!(\"hello\");".to_string(),
        "}".to_string(),
        "".to_string(),
        "// Another comment".to_string(),
        "fn helper() {".to_string(),
        "}".to_string(),
    ];

    group.bench_function("small_file_10_lines", |b| {
        b.iter(|| classify_file(black_box(&small_file), rust_lang))
    });

    // Medium file (100 lines)
    let mut medium_file = Vec::new();
    for i in 0..100 {
        if i % 10 == 0 {
            medium_file.push("".to_string());
        } else if i % 5 == 0 {
            medium_file.push(format!("// Comment {}", i));
        } else {
            medium_file.push(format!("let x{} = {};", i, i));
        }
    }

    group.bench_function("medium_file_100_lines", |b| {
        b.iter(|| classify_file(black_box(&medium_file), rust_lang))
    });

    // Large file (1000 lines)
    let mut large_file = Vec::new();
    for i in 0..1000 {
        if i % 10 == 0 {
            large_file.push("".to_string());
        } else if i % 5 == 0 {
            large_file.push(format!("// Comment {}", i));
        } else {
            large_file.push(format!("let x{} = {};", i, i));
        }
    }

    group.bench_function("large_file_1000_lines", |b| {
        b.iter(|| classify_file(black_box(&large_file), rust_lang))
    });

    group.finish();
}

fn bench_language_detection(c: &mut Criterion) {
    let detector = LanguageDetector::new();

    let mut group = c.benchmark_group("language_detection");

    let test_paths = vec![
        ("main.rs", "Rust"),
        ("script.py", "Python"),
        ("app.js", "JavaScript"),
        ("component.tsx", "TypeScript"),
        ("index.html", "HTML"),
        ("styles.css", "CSS"),
        ("config.toml", "TOML"),
        ("data.json", "JSON"),
    ];

    for (path, lang_name) in test_paths {
        group.bench_with_input(
            BenchmarkId::from_parameter(lang_name),
            &path,
            |b, path_str| {
                let path = Path::new(path_str);
                b.iter(|| detector.detect_from_path(black_box(path)))
            },
        );
    }

    group.finish();
}

fn bench_stats_aggregation(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats_aggregation");

    // Benchmark adding stats to ProjectStats
    group.bench_function("add_file_stats", |b| {
        b.iter(|| {
            let mut project = ProjectStats::new();
            for i in 0..100 {
                let lang = if i % 3 == 0 {
                    "Rust"
                } else if i % 3 == 1 {
                    "Python"
                } else {
                    "JavaScript"
                };
                project.add_file_stats(
                    lang,
                    FileStats {
                        blank: 10,
                        comment: 20,
                        code: 70,
                    },
                );
            }
            black_box(project)
        })
    });

    // Benchmark getting totals
    group.bench_function("calculate_totals", |b| {
        let mut project = ProjectStats::new();
        for i in 0..100 {
            let lang = if i % 3 == 0 {
                "Rust"
            } else if i % 3 == 1 {
                "Python"
            } else {
                "JavaScript"
            };
            project.add_file_stats(
                lang,
                FileStats {
                    blank: 10,
                    comment: 20,
                    code: 70,
                },
            );
        }

        b.iter(|| black_box(project.total()))
    });

    // Benchmark sorting languages
    group.bench_function("get_sorted_languages", |b| {
        let mut project = ProjectStats::new();
        for i in 0..20 {
            project.add_file_stats(
                &format!("Language{}", i),
                FileStats {
                    blank: 10,
                    comment: 20,
                    code: 70,
                },
            );
        }

        b.iter(|| black_box(project.get_languages()))
    });

    group.finish();
}

fn bench_file_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_processing");

    // Create a temporary Rust file
    let mut temp_file = NamedTempFile::new().unwrap();
    for i in 0..100 {
        if i % 10 == 0 {
            writeln!(temp_file, "").unwrap();
        } else if i % 5 == 0 {
            writeln!(temp_file, "// Comment {}", i).unwrap();
        } else {
            writeln!(temp_file, "let x{} = {};", i, i).unwrap();
        }
    }
    temp_file.flush().unwrap();

    // Rename to .rs extension
    let temp_path = temp_file.path().with_extension("rs");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    group.bench_function("process_rust_file_100_lines", |b| {
        let processor = FileProcessor::new();
        b.iter(|| processor.process_file(black_box(&temp_path)))
    });

    group.finish();

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

criterion_group!(
    benches,
    bench_line_classification,
    bench_file_classification,
    bench_language_detection,
    bench_stats_aggregation,
    bench_file_processing
);
criterion_main!(benches);
