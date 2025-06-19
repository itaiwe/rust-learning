use std::env;
use std::process;

use minigrep_learn::Config;

fn handle_error(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| handle_error(&format!("Problem parsing arguments: {err}")));

    if let Err(e) = minigrep_learn::run_program(config) {
        handle_error(&format!("Application error: {e}"));
    }
}
