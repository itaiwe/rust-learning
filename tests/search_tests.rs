use std::vec;

use clap::Parser;
use minigrep_learn::*;

#[test]
fn case_sensitive_contained() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    let args = vec![
        "minigrep".to_string(),
        query.to_string(),
        "path".to_string(), // file path is irrelevant for testing search
        "--mode=Contains".to_string(),
    ];

    let configuration = Args::try_parse_from(args).expect("Failed to parse args for test");

    assert_eq!(
        vec!["safe, fast, productive."],
        search(&configuration, contents)
    );
}

#[test]
fn case_insensitive_contained() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let args = vec![
        "minigrep".to_string(),
        query.to_string(),
        "path".to_string(), // file path is irrelevant for testing search
        "--mode=Contains".to_string(),
        "-i".to_string(),
    ];

    let configuration = Args::try_parse_from(args).expect("Failed to parse args for test");

    assert_eq!(vec!["Rust:", "Trust me."], search(&configuration, contents));
}

#[test]
fn case_sensitive_exact() {
    let query = "Duct tape.";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    let args = vec![
        "minigrep".to_string(),
        query.to_string(),
        "path".to_string(), // file path is irrelevant for testing search
        "--mode=Exact".to_string(),
    ];

    let configuration = Args::try_parse_from(args).expect("Failed to parse args for test");

    assert_eq!(vec!["Duct tape."], search(&configuration, contents));
}

#[test]
fn case_insensitive_exact() {
    let query = "rUsT:";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let args = vec![
        "minigrep".to_string(),
        query.to_string(),
        "path".to_string(), // file path is irrelevant for testing search
        "--mode=Exact".to_string(),
        "-i".to_string(),
    ];

    let configuration = Args::try_parse_from(args).expect("Failed to parse args for test");

    assert_eq!(vec!["Rust:"], search(&configuration, contents));
}
