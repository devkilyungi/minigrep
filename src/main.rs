use std::{env, process};

use minigrep::config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Error: Not enough arguments\n");
        minigrep::core::print_help();
        process::exit(1);
    }

    let config = config::parse_args(&args).unwrap_or_else(|err| {
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
