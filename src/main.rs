use sniffy::cli::Cli;
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
