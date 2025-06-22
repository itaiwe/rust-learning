use clap::Parser;
use owo_colors::OwoColorize;
use std::error::Error;
use std::fs;
use std::process;
use strip_ansi_escapes;
use strum::{Display, EnumString};
use tabled::{
    settings::{disable::Remove, object::Rows, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct ResultRow {
    line: String,
}

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

    let table_output = format_results(&results, &args.query);

    println!(
        "Found {} matches in {} ms",
        results.len(),
        duration.num_milliseconds()
    );

    if args.copy {
        let clipboard_result = String::from_utf8(strip_ansi_escapes::strip(&table_output))
            .unwrap_or_else(|_| handle_error("Failed to strip ANSI escapes from table output"));
        use arboard::Clipboard;
        Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(clipboard_result))
            .unwrap_or_else(|err| handle_error(&format!("Failed to copy to clipboard: {err}")))
    }

    println!("{}", table_output);

    Ok(())
}

/// Searches for lines in `contents` that contain `args.query`.
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

fn format_results<'a>(results: &[&'a str], query: &str) -> String {
    let colored_rows: Vec<ResultRow> = results
        .iter()
        .map(|line| {
            let highlighted = line.replace(query, &query.red().bold().to_string());
            ResultRow { line: highlighted }
        })
        .collect();

    // Make tabled measure the length of table end without ANSI escape codes
    let max_width = colored_rows
        .iter()
        .map(|row| {
            let stripped = strip_ansi_escapes::strip(&row.line);

            String::from_utf8_lossy(&stripped).chars().count()
        })
        .max()
        .unwrap_or(0);

    // Pad all lines to max width
    let padded_lines: Vec<String> = colored_rows
        .iter()
        .map(|row| {
            let stripped = strip_ansi_escapes::strip(&row.line);
            let visible_len = String::from_utf8_lossy(&stripped).chars().count();
            let padding = " ".repeat(max_width.saturating_sub(visible_len));
            format!("{line}{padding}", line = row.line)
        })
        .collect();

    Table::new(padded_lines)
        .with(Remove::row(Rows::first()))
        .with(Style::modern())
        .to_string()
}
