use rayon::prelude::*;
use sniffy::cli::Cli;
use sniffy::git::GitAnalyzer;
use sniffy::output::OutputFormatter;
use sniffy::processor::FileProcessor;
use sniffy::stats::ProjectStats;
use sniffy::walker::DirectoryWalker;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    // Parse and validate CLI arguments
    let cli = Cli::parse_args();

    if let Err(e) = cli.validate() {
        eprintln!("Error: {}", e);
        process::exit(2);
    }

    // Handle history mode
    if cli.history {
        run_history_mode(&cli);
        return;
    }

    // Configure Rayon thread pool
    if cli.jobs > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(cli.jobs)
            .build_global()
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to set thread count: {}", e);
            });
    }

    // Collect all file paths first
    let mut all_files = Vec::new();
    for path in &cli.paths {
        if cli.verbose {
            eprintln!("Scanning: {}", path.display());
        }

        let walker = DirectoryWalker::new(path).hidden(cli.hidden);
        all_files.extend(walker.walk());
    }

    let total_files = all_files.len();

    if cli.verbose {
        eprintln!("Found {} files, processing in parallel...", total_files);
    }

    // Process files in parallel
    let processed_count = Arc::new(AtomicUsize::new(0));
    let project_stats = all_files
        .par_iter()
        .map(|file_path| {
            let processor = FileProcessor::new();
            let mut local_stats = ProjectStats::new();

            if let Some((language, stats)) = processor.process_file(file_path) {
                local_stats.add_file_stats(&language, stats);

                // Update progress counter
                let count = processed_count.fetch_add(1, Ordering::Relaxed) + 1;
                if cli.verbose && count.is_multiple_of(100) {
                    eprintln!("Processed {} files...", count);
                }
            }

            local_stats
        })
        .reduce(ProjectStats::new, |mut acc, stats| {
            acc.merge(stats);
            acc
        });

    let processed_files = processed_count.load(Ordering::Relaxed);

    if cli.verbose {
        eprintln!(
            "Total files scanned: {}, processed: {}",
            total_files, processed_files
        );
    }

    // Format and print results based on format option
    let format_lower = cli.format.to_lowercase();
    match format_lower.as_str() {
        "json" => match OutputFormatter::format_json(&project_stats) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Error formatting JSON: {}", e);
                process::exit(1);
            }
        },
        "csv" => {
            let csv = OutputFormatter::format_csv(&project_stats);
            println!("{}", csv);
        }
        _ => {
            // Default to table format
            let use_color = cli.should_use_color();
            let table = OutputFormatter::format_table(&project_stats, use_color);
            println!("{}", table);
        }
    }
}

fn run_history_mode(cli: &Cli) {
    // Use the first path (or current directory if none specified)
    let path = cli.paths.first().expect("At least one path required");

    // Check if it's a git repository
    if !GitAnalyzer::is_git_repo(path) {
        eprintln!("Error: {} is not in a git repository", path.display());
        process::exit(1);
    }

    // Create GitAnalyzer
    let analyzer = match GitAnalyzer::new(path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: Failed to open git repository: {}", e);
            process::exit(1);
        }
    };

    // Parse since and until dates
    let since = match cli.parse_since_date() {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    };

    let until = match cli.parse_until_date() {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    };

    if cli.verbose {
        match (since, until) {
            (Some(since_date), Some(until_date)) => {
                eprintln!("Analyzing commits from {} to {}", since_date, until_date);
            }
            (Some(since_date), None) => {
                eprintln!("Analyzing commits since {}", since_date);
            }
            (None, Some(until_date)) => {
                eprintln!("Analyzing commits until {}", until_date);
            }
            (None, None) => {
                eprintln!("Analyzing all commits");
            }
        }
    }

    // Analyze history
    let stats = match analyzer.analyze_history(since, until, cli.verbose) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed to analyze git history: {}", e);
            process::exit(1);
        }
    };

    // Filter by author if specified
    let stats = if let Some(author_filter) = &cli.author {
        // Filter daily stats and by_author to only include the specified author
        let filtered_by_author = stats
            .by_author
            .into_iter()
            .filter(|(name, _)| name.contains(author_filter))
            .collect();

        sniffy::git::HistoricalStats {
            daily: stats.daily,
            by_author: filtered_by_author,
            total_commits: stats.total_commits,
        }
    } else {
        stats
    };

    // Aggregate by week if requested
    let (time_series, period_label, limit) = if cli.by_week {
        let weekly = stats.aggregate_by_week();
        (weekly, "Weekly", Some(12)) // Show last 12 weeks by default
    } else {
        (stats.daily.clone(), "Daily", Some(30)) // Show last 30 days by default
    };

    // Format and print results based on format option
    let format_lower = cli.format.to_lowercase();
    match format_lower.as_str() {
        "json" => match OutputFormatter::format_history_json(&stats, &time_series, period_label) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Error formatting JSON: {}", e);
                process::exit(1);
            }
        },
        "csv" => {
            let csv = OutputFormatter::format_history_csv(&stats, &time_series, period_label);
            println!("{}", csv);
        }
        _ => {
            // Default to table format
            let use_color = cli.should_use_color();
            let output = OutputFormatter::format_history(&stats, &time_series, period_label, limit, use_color);
            println!("{}", output);
        }
    }
}
