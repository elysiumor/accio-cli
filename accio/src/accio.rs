use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Accio CLI - Fast and elegant Rust-based file utility tool.
#[derive(Parser)]
#[command(
    name = "Accio",
    version = "1.0",
    author = "Your Name",
    about = "A modern and optimized Rust CLI file tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List files in the current directory
    Ls,
    /// Print the current working directory
    Pwd,
    /// Search for a file within a directory (supports parallel mode)
    Search {
        /// Filename to search for
        filename: String,
    },
}

// Parallel recursive search using Rayon
fn search_files_parallel(
    path: &PathBuf,
    target: &str,
    results: Arc<Mutex<Vec<PathBuf>>>,
    pb: Arc<ProgressBar>,
) {
    let entries = match fs::read_dir(path) {
        Ok(rd) => rd.collect::<Vec<_>>(),
        Err(_) => return,
    };

    let (dirs, files): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .filter_map(|e| e.ok())
        .partition(|e| e.path().is_dir());

    for entry in files {
        if let Some(name) = entry.file_name().to_str() {
            if name.eq_ignore_ascii_case(target) {
                let mut res = results.lock().unwrap();
                res.push(entry.path());
            }
        }
    }

    pb.inc(dirs.len() as u64);

    dirs.into_par_iter().for_each(|subdir| {
        search_files_parallel(
            &subdir.path(),
            target,
            Arc::clone(&results),
            Arc::clone(&pb),
        );
    });
}

// Sequential recursive search
fn search_files_sequential(
    path: &PathBuf,
    target: &str,
    results: &mut Vec<PathBuf>,
    pb: &ProgressBar,
) {
    let entries = match fs::read_dir(path) {
        Ok(rd) => rd.collect::<Vec<_>>(),
        Err(_) => return,
    };

    let (dirs, files): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .filter_map(|e| e.ok())
        .partition(|e| e.path().is_dir());

    for entry in files {
        if let Some(name) = entry.file_name().to_str() {
            if name.eq_ignore_ascii_case(target) {
                results.push(entry.path());
            }
        }
    }

    pb.inc(dirs.len() as u64);

    for dir in dirs {
        search_files_sequential(&dir.path(), target, results, pb);
    }
}

// Utility: format time as "X m Y secs"
fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{}mins {}secs", minutes, seconds)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pwd => {
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            println!("{}", current_dir.display());
        }

        Commands::Ls => {
            let current_dir = std::env::current_dir().expect("Failed to get directory contents");
            println!("Contents of {}:", current_dir.display());
            for entry in fs::read_dir(current_dir).expect("Cannot read directory") {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }

        Commands::Search { filename } => {
            print!("Enter directory path to scan: ");
            io::stdout().flush().unwrap();

            let mut directory = String::new();
            io::stdin()
                .read_line(&mut directory)
                .expect("Failed to read input");
            let directory = directory.trim();
            let root_path = PathBuf::from(directory);

            if !root_path.exists() || !root_path.is_dir() {
                eprintln!("The path '{}' is not a valid directory.", directory);
                return;
            }

            println!("Use parallel search (Rayon)? (y/n): ");
            io::stdout().flush().unwrap();

            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            let use_parallel = matches!(response.trim().to_lowercase().as_str(), "y" | "yes");

            let pb = Arc::new(ProgressBar::new_spinner());
            pb.set_style(
                ProgressStyle::with_template("{spinner} Directories scanned: {pos}")
                    .unwrap()
                    .tick_chars("/|\\- "),
            );

            let start_time = Instant::now(); // Start timer

            if use_parallel {
                println!("Starting parallel search...");
                let results = Arc::new(Mutex::new(Vec::new()));
                search_files_parallel(&root_path, filename, Arc::clone(&results), Arc::clone(&pb));
                pb.finish_and_clear();

                let elapsed = start_time.elapsed();
                let formatted_time = format_duration(elapsed);

                let final_results = results.lock().unwrap();
                if final_results.is_empty() {
                    println!(
                        "No file named '{}' found. (Completed in {})",
                        filename, formatted_time
                    );
                } else {
                    println!(
                        "\x1b[32mFound the following files:\x1b[0m (Completed in {})",
                        formatted_time
                    );
                    for path in final_results.iter() {
                        println!("{}", path.display());
                    }
                }
            } else {
                println!("Starting sequential search...");
                let mut results = Vec::new();
                search_files_sequential(&root_path, filename, &mut results, &pb);
                pb.finish_and_clear();

                let elapsed = start_time.elapsed();
                let formatted_time = format_duration(elapsed);

                if results.is_empty() {
                    println!(
                        "No file named '{}' found. (Completed in {})",
                        filename, formatted_time
                    );
                } else {
                    println!(
                        "\x1b[32mFound the following files:\x1b[0m (Completed in {})",
                        formatted_time
                    );
                    for path in results {
                        println!("{}", path.display());
                    }
                }
            }
        }
    }
}
