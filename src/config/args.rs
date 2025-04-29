//! Command-line argument parsing for minigrep.
use crate::{
    core,
    models::{Config, ConfigError, ContextFlag},
};
use std::{env, mem, path, process};

/// Parses command-line arguments into a Config object.
///
/// # Arguments
///
/// * `args` - An iterator of command-line arguments
///
/// # Returns
///
/// * `Result<Config, ConfigError>` - A valid configuration or an error message
///
/// # Errors
///
/// Returns an error if:
/// - Not enough arguments are provided
/// - Too many arguments are provided
/// - Invalid flags or arguments are specified
pub fn parse_args<I>(args: I) -> Result<Config, ConfigError>
where
    I: Iterator<Item = String>,
{
    let mut args_vec: Vec<String> = args.collect();

    if args_vec.len() <= 1 {
        println!("Error: Not enough arguments\n");
        core::print_help();
        process::exit(1);
    }

    if args_vec.len() > 1 && (args_vec[1] == "--help" || args_vec[1] == "-h") {
        core::print_help();
        process::exit(0);
    }

    if args_vec.len() > 1 && (args_vec[1] == "--version" || args_vec[1] == "-v") {
        println!("minigrep {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if args_vec.len() < 3 {
        return Err(ConfigError::NotEnoughArguments);
    }

    // Core arguments
    let query = mem::take(&mut args_vec[1]);
    let file_path_1 = mem::take(&mut args_vec[2]);

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

    let mut i = 3;
    while i < args_vec.len() {
        let arg = &args_vec[i];

        match arg.as_str() {
            // Case sensitivity flags
            "-ic" => ignore_case = true,
            "-cs" => ignore_case = false,

            // Stats flag
            "--stats" | "--s" => show_stats = true,

            // Recursive flag
            "--recursive" | "--r" => recursive = true,

            // Context flags
            "--before" | "--b" => {
                context_flag = ContextFlag::Before;
                // Check for context count in next argument
                if i + 1 < args_vec.len() {
                    if let Ok(count) = args_vec[i + 1].parse::<u8>() {
                        context_count = count;
                        i += 1; // Skip the next argument
                    } else {
                        context_count = 1; // Default count
                    }
                } else {
                    context_count = 1; // Default count
                }
            }

            "--after" | "--a" => {
                context_flag = ContextFlag::After;
                // Check for context count in next argument
                if i + 1 < args_vec.len() {
                    if let Ok(count) = args_vec[i + 1].parse::<u8>() {
                        context_count = count;
                        i += 1; // Skip the next argument
                    } else {
                        context_count = 1; // Default count
                    }
                } else {
                    context_count = 1; // Default count
                }
            }

            "--context" | "--c" => {
                context_flag = ContextFlag::Context;
                // Check for context count in next argument
                if i + 1 < args_vec.len() {
                    if let Ok(count) = args_vec[i + 1].parse::<u8>() {
                        context_count = count;
                        i += 1; // Skip the next argument
                    } else {
                        context_count = 1; // Default count
                    }
                } else {
                    context_count = 1; // Default count
                }
            }

            // If we get here, assume it's a second file path if it's empty
            _ if file_path_2.is_empty() && !arg.starts_with("-") => {
                file_path_2 = std::mem::take(&mut args_vec[i]);
            }

            // Unknown argument
            _ => return Err(ConfigError::InvalidArgument(arg.clone())),
        }

        i += 1;
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
