//! Entry point for the minigrep command-line tool.

use minigrep::config;
use std::{env, process};

/// Program entry point.
///
/// Parses command-line arguments, configures and runs the search operation,
/// and handles any errors that occur.
fn main() {
    let config = config::parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        println!("\nFor help, use --help or -h");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        println!("\nTo see available options, use --help or -h");
        process::exit(1);
    }
}
