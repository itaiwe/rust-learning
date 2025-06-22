use clap::Parser;
use owo_colors::OwoColorize;
use std::error::Error;
use std::fs;
use std::process;
use strum::{Display, EnumString};

#[derive(Parser, Debug)]
#[command(
    name = "minigrep",
    author = "Itai Weiner",
    version = "0.2.0",
    about = "A simple grep-like program"
)]
/// Defines arguments for minigrep command
///
/// # Fields
///
/// - `query` (`String`) - query string to search for
/// - `file_path` (`String`) - file path to search within
/// - `ignore_case` (`bool`) - flag denoting whether or not to ignore case when searching
/// - `mode` (`SearchMode`) - flag denoting whether query should be contained inside lines or match an exact line in file contents
/// - `copy` (`bool`) - flag denoting whether to copy results to clipboard or not
pub struct Args {
    #[arg(required = true)]
    pub query: String,

    #[arg(required = true)]
    pub file_path: String,

    #[arg(short, long)]
    pub ignore_case: bool,

    #[arg(short, long, default_value = "Contains")]
    pub mode: SearchMode,

    #[arg(long)]
    pub copy: bool,
}

#[derive(EnumString, Display, Debug, Clone)]
/// Defines search mode of query in contents
///
/// # Variants
///
/// - `Exact` - Exact match, i.e. query appears as full line
/// - `Contains` - Appears as part of line
pub enum SearchMode {
    Exact,
    Contains,
}

/// Prints error to stderr and terminates process.
///
/// # Arguments
///
/// - `message` (`&str`) - Error message.
pub fn handle_error(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}

/// Runs the program with the given configuration.
///
/// # Arguments
///
/// - `args` (`Args`) - configuration for the program, including the query and file path.
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - result indicating success or failure.
///
/// # Errors
///
/// File reading errors or other runtime errors will return an `Err` variant with a boxed error type.
pub fn run_program(args: &Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file_path)?;

    let start_time = chrono::Utc::now();

    let results = search(&args, &contents);

    let duration = chrono::Utc::now() - start_time;

    let output = format_results(&results, &args.query);

    println!(
        "Found {} matches in {} ms",
        results.len(),
        duration.num_milliseconds()
    );

    if args.copy {
        use arboard::Clipboard;
        Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(output.join("\n")))
            .unwrap_or_else(|err| handle_error(&format!("Failed to copy to clipboard: {err}")))
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Searches for lines in `contents` that contain the `query`.
///
/// # Arguments
///
/// - `args` (`&Args`) - arguments struct of command (containing query, search mode and ignore case needed for function).
/// - `contents` (`&'a str`) - contents of the file to search within.
///
/// # Returns
///
/// - `Vec<&'a str>` - Results containing lines that match the query.
pub fn search<'a>(args: &Args, contents: &'a str) -> Vec<&'a str> {
    let query = if args.ignore_case {
        args.query.to_lowercase()
    } else {
        args.query.clone()
    };

    contents
        .lines()
        .filter(|line| {
            // Creating new String variable because to_lowercase returns String instead of &str
            let line_to_compare = if args.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            match args.mode {
                SearchMode::Exact => line_to_compare == query,
                SearchMode::Contains => line_to_compare.contains(&query),
            }
        })
        .collect()
}

pub fn format_results<'a>(results: &Vec<&'a str>, query: &str) -> Vec<String> {
    results
        .into_iter()
        .map(|line| line.replace(query, &query.red().bold().to_string()))
        .collect()
}
