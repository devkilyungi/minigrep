use crate::{
    core,
    models::{Config, ConfigError, ContextFlag},
};
use std::{env, path, process};

pub fn parse_args(args: &[String]) -> Result<Config, ConfigError> {
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        core::print_help();
        process::exit(0);
    }

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("minigrep {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if args.len() < 3 {
        return Err(ConfigError::NotEnoughArguments);
    }

    // Core arguments
    let query = args[1].clone();
    let file_path_1 = args[2].clone();

    // Default values
    let mut file_path_2 = "".to_string();
    let mut ignore_case = env::var("IGNORE_CASE").is_ok();
    let mut context_flag = ContextFlag::After;
    let mut context_count = 0;
    let mut show_stats = false;
    let mut recursive = false;

    // Process remaining arguments
    // Supported formats:
    // 1. minigrep <query> <file>
    // 2. minigrep <query> <file> -ic/--cs (case options)
    // 3. minigrep <query> <file> --stats/--s (statistics)
    // 4. minigrep <query> <file> --context/--c/--before/--b/--after/--a [count]
    // 5. minigrep <query> <directory> --recursive/--r (recursive search)
    // 6. minigrep <query> <file1> <file2> (multiple files)
    //
    // All these options can be combined in any order after the query and first file

    for i in 3..args.len() {
        let arg = &args[i];

        // Handle case sensitivity flags
        if arg == "-ic" {
            ignore_case = true;
            continue;
        } else if arg == "-cs" {
            ignore_case = false;
            continue;
        }

        // Handle stats flag
        if arg == "--stats" || arg == "--s" {
            show_stats = true;
            continue;
        }

        // Handle recursive flag
        if arg == "--recursive" || arg == "--r" {
            recursive = true;
            continue;
        }

        // Handle context flags
        if arg == "--before" || arg == "--b" {
            context_flag = ContextFlag::Before;
            // Look for context count in next argument
            if i + 1 < args.len() {
                if let Ok(count) = args[i + 1].parse::<u8>() {
                    context_count = count;
                    // Skip the next argument since we consumed it
                    continue;
                }
            }
            context_count = 1; // Default count if not specified
            continue;
        } else if arg == "--after" || arg == "--a" {
            context_flag = ContextFlag::After;
            if i + 1 < args.len() {
                if let Ok(count) = args[i + 1].parse::<u8>() {
                    context_count = count;
                    continue;
                }
            }
            context_count = 1;
            continue;
        } else if arg == "--context" || arg == "--c" {
            context_flag = ContextFlag::Context;
            if i + 1 < args.len() {
                if let Ok(count) = args[i + 1].parse::<u8>() {
                    context_count = count;
                    continue;
                }
            }
            context_count = 1;
            continue;
        }

        // If it's a number following a context flag, we've already handled it
        if arg.parse::<u8>().is_ok()
            && i > 3
            && (args[i - 1] == "--before"
                || args[i - 1] == "--b"
                || args[i - 1] == "--after"
                || args[i - 1] == "--a"
                || args[i - 1] == "--context"
                || args[i - 1] == "--c")
        {
            continue;
        }

        // If we get here, assume it's a second file path if it's empty
        if file_path_2.is_empty() && !arg.starts_with("-") {
            file_path_2 = arg.clone();
            continue;
        }

        // If we get here, it's an unknown argument
        return Err(ConfigError::InvalidArgument(arg.clone()));
    }

    // Verify directory if recursive
    if recursive {
        let path = path::Path::new(&file_path_1);
        if !path.is_dir() {
            return Err(ConfigError::NotADirectory(file_path_1));
        }
    }

    Ok(Config {
        query,
        file_path_1,
        file_path_2,
        ignore_case,
        context_flag,
        context_count,
        show_stats,
        recursive,
    })
}
