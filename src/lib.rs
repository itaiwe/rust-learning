use std::env;
use std::error::Error;
use std::fs;

/// Runs the program with the given configuration.
///
/// # Arguments
///
/// - `config` (`Config`) - configuration for the program, including the query and file path.
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - result indicating success or failure.
///
/// # Errors
///
/// File reading errors or other runtime errors will return an `Err` variant with a boxed error type.
pub fn run_program(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Configuration struct for the program.
///
/// # Fields
///
/// - `query` (`String`) - query string to search for in the file.
/// - `file_path` (`String`) - file path to search within.
/// - `ignore_case` (`bool`) - flag indicating whether the search should be case-insensitive.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Constructor for `Config`
    ///
    /// # Arguments
    ///
    /// - `args` (`&[String]`) - arguments given in command line.
    ///
    /// # Returns
    ///
    /// - `Config` - command configuration.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <program> <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Searches for lines in `contents` that contain the `query`.
/// 
/// # Arguments
/// 
/// - `query` (`&str`) - query string to search for.
/// - `contents` (`&'a str`) - contents of the file to search within.
/// 
/// # Returns
/// 
/// - `Vec<&'a str>` - Results containing lines that match the query.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Searches for lines in `contents` that contain the `query`, ignoring case.
/// 
/// # Arguments
/// 
/// - `query` (`&str`) - query string to search for.
/// - `contents` (`&'a str`) - contents of the file to search within.
/// 
/// # Returns
/// 
/// - `Vec<&'a str>` - Results containing lines that match the query, ignoring case.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
