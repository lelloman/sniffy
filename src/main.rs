use sniffy::cli::Cli;
use sniffy::git::GitAnalyzer;
use sniffy::output::OutputFormatter;
use sniffy::processor::FileProcessor;
use sniffy::stats::ProjectStats;
use sniffy::walker::DirectoryWalker;
use std::process;

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

    // Create file processor
    let processor = FileProcessor::new();
    let mut project_stats = ProjectStats::new();

    let mut total_files = 0;
    let mut processed_files = 0;

    // Process each path
    for path in &cli.paths {
        if cli.verbose {
            eprintln!("Analyzing: {}", path.display());
        }

        // Create directory walker
        let walker = DirectoryWalker::new(path).hidden(cli.hidden);

        // Walk and process each file
        for file_path in walker.walk() {
            total_files += 1;

            if cli.verbose && total_files % 100 == 0 {
                eprintln!("Processed {} files...", total_files);
            }

            // Process the file
            if let Some((language, stats)) = processor.process_file(&file_path) {
                project_stats.add_file_stats(&language, stats);
                processed_files += 1;
            }
        }
    }

    if cli.verbose {
        eprintln!(
            "Total files scanned: {}, processed: {}",
            total_files, processed_files
        );
    }

    // Format and print results
    let table = OutputFormatter::format_table(&project_stats);
    println!("{}", table);
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

    // Parse since date
    let since = match cli.parse_since_date() {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
    };

    if cli.verbose {
        if let Some(since_date) = since {
            eprintln!("Analyzing commits since {}", since_date);
        } else {
            eprintln!("Analyzing all commits");
        }
    }

    // Analyze history
    let stats = match analyzer.analyze_history(since) {
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

    // Determine display limit (default to 30 days)
    let limit = if cli.by_week {
        // TODO: Implement weekly aggregation in future
        Some(30)
    } else {
        Some(30) // Show last 30 days by default
    };

    // Format and print results
    let output = OutputFormatter::format_history(&stats, limit);
    println!("{}", output);
}
