use clap::Parser;
use minigrep_learn::Args;

fn main() {
    let args = Args::try_parse();

    match args {
        Ok(_) => {
            if let Err(e) = minigrep_learn::run_program(&args.unwrap()) {
                minigrep_learn::handle_error(&format!("Application error: {e}"));
            }
        },
        Err(_) => minigrep_learn::handle_error("Usage: minigrep_learn <query> <file_path> [-i|--ignore-case] [-m <mode>|--mode=<mode>] [--copy]"),
    }
}
