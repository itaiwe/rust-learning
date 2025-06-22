use std::env;
use std::process;

use minigrep_learn::Config;

/// Prints error to stderr and terminates process.
///
/// # Arguments
///
/// - `message` (`&str`) - Error message.
fn handle_error(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| handle_error(&format!("Problem parsing arguments: {err}")));

    if let Err(e) = minigrep_learn::run_program(config) {
        handle_error(&format!("Application error: {e}"));
    }
}
