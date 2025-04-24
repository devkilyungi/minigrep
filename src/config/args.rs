use crate::{
    core,
    models::{Config, ConfigError, ContextFlag},
};
use std::env;

pub fn parse_args(args: &[String]) -> Result<Config, ConfigError> {
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        core::print_help();
        std::process::exit(0);
    }

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("minigrep {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    if args.len() < 3 {
        return Err(ConfigError::NotEnoughArguments);
    }

    let query = args[1].clone();
    let file_path_1 = args[2].clone();
    let mut file_path_2 = "".to_string();
    let mut ignore_case = env::var("IGNORE_CASE").is_ok();
    let mut context_flag = ContextFlag::After;
    let mut context_count = 0;
    let mut show_stats = false;
    let mut recursive = false;

    match args.len() {
        3 => {
            // Format: [binary, query, file_path_1/directory]
            // Example: minigrep "pattern" file.txt
            // ignore_case defaults to env var setting
        }
        4 => {
            // Format: [binary, query, file_path_1/directory, option]
            // Options can be:
            // 1. Context flag: --before, --after, --context, --stats
            // 2. Recursive flag: --recursive, --r
            // 3. Case sensitivity flag: -ic, -cs
            // 4. Second file path
            // Examples:
            // minigrep "pattern" file.txt --context
            // minigrep "pattern" dir/ --recursive
            // minigrep "pattern" file.txt -ic
            // minigrep "pattern" file1.txt file2.txt

            let fourth = args[3].clone();

            if fourth.starts_with("--") {
                if fourth == "--recursive" || fourth == "--r" {
                    recursive = true;
                } else {
                    // it's a context flag
                    context_count = 1;
                    context_flag = match fourth.as_str() {
                        "--before" | "--b" => ContextFlag::Before,
                        "--after" | "--a" => ContextFlag::After,
                        "--context" | "--c" => ContextFlag::Context,
                        "--stats" | "--s" => {
                            show_stats = true;
                            ContextFlag::Stats
                        }
                        _ => return Err(ConfigError::InvalidContextFlag(fourth)),
                    };
                }
            } else if fourth.starts_with('-') {
                // it's a flag
                ignore_case = match fourth.as_str() {
                    "-ic" => true,
                    "-cs" => false,
                    // ignore_case already set from env
                    _ => return Err(ConfigError::InvalidCaseFlag(fourth)),
                };
            } else {
                // it's a second file
                file_path_2 = fourth;
            }
        }
        5 => {
            // Format: [binary, query, file_path_1/directory, option1, option2]
            // Common combinations:
            // 1. [query, dir, --recursive, --stats/--ic/-cs]
            // 2. [query, file1, file2, --context/--stats/-ic/-cs]
            // 3. [query, file, --context/--before/--after, context_count]
            // Examples:
            // minigrep "pattern" dir/ --recursive --stats
            // minigrep "pattern" file1.txt file2.txt -ic
            // minigrep "pattern" file.txt --context 2

            let fourth = args[3].clone();
            let fifth = args[4].clone();

            if fifth.starts_with("--") {
                if fourth == "--recursive" || fourth == "--r" {
                    recursive = true;
                    // it's a stats flag
                    context_flag = match fifth.as_str() {
                        "--stats" | "--s" => {
                            show_stats = true;
                            ContextFlag::Stats
                        }
                        _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                    };
                } else {
                    file_path_2 = fourth;
                    // it's a context flag
                    context_count = 1;
                    context_flag = match fifth.as_str() {
                        "--before" | "--b" => ContextFlag::Before,
                        "--after" | "--a" => ContextFlag::After,
                        "--context" | "--c" => ContextFlag::Context,
                        "--stats" | "--s" => {
                            show_stats = true;
                            ContextFlag::Stats
                        }
                        _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                    };
                }
            } else if fifth.starts_with('-') {
                if fourth == "--recursive" || fourth == "--r" {
                    recursive = true;
                } else {
                    file_path_2 = fourth;
                }
                // it's a flag
                ignore_case = match fifth.as_str() {
                    "-ic" => true,
                    "-cs" => false,
                    // ignore_case already set from env
                    _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
                };
            } else {
                // it's a context count
                context_flag = match fourth.as_str() {
                    "--before" | "--b" => ContextFlag::Before,
                    "--after" | "--a" => ContextFlag::After,
                    "--context" | "--c" => ContextFlag::Context,
                    "--stats" | "--s" => {
                        show_stats = true;
                        ContextFlag::Stats
                    }
                    _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                };
                context_count = match fifth.parse() {
                    Ok(count) => count,
                    Err(_) => return Err(ConfigError::InvalidContextCount(fifth)),
                };
            }
        }
        6 => {
            // Format: [binary, query, file_path_1/directory, option1, option2, option3]
            // Common combinations:
            // 1. [query, dir, --recursive, -ic/-cs, --stats]
            // 2. [query, dir, --recursive, --context/--before/--after, context_count]
            // 3. [query, file1, file2, -ic/-cs, --context/--before/--after]
            // 4. [query, file, -ic/-cs, --context/--before/--after, context_count]
            // Examples:
            // minigrep "pattern" dir/ --recursive -ic --stats
            // minigrep "pattern" dir/ --recursive --context 2
            // minigrep "pattern" file1.txt file2.txt -ic --context

            let fourth = args[3].clone();
            let fifth = args[4].clone();
            let sixth = args[5].clone();

            if fourth == "--recursive" || fourth == "--r" {
                recursive = true;

                // Fifth arg could be case sensitivity or context flag
                if fifth.starts_with('-') {
                    if fifth == "-ic" {
                        ignore_case = true;
                    } else if fifth == "-cs" {
                        ignore_case = false;
                    } else if fifth.starts_with("--") {
                        // It's a context flag
                        context_count = 1;
                        context_flag = match fifth.as_str() {
                            "--before" | "--b" => ContextFlag::Before,
                            "--after" | "--a" => ContextFlag::After,
                            "--context" | "--c" => ContextFlag::Context,
                            "--stats" | "--s" => {
                                show_stats = true;
                                ContextFlag::Stats
                            }
                            _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                        };
                    } else {
                        return Err(ConfigError::InvalidArgument(fifth));
                    }
                } else {
                    return Err(ConfigError::InvalidArgument(fifth));
                }

                // Sixth arg could be stats, context count, or case sensitivity
                if sixth.starts_with("--") {
                    if sixth == "--stats" || sixth == "--s" {
                        show_stats = true;
                    } else {
                        return Err(ConfigError::InvalidArgument(sixth));
                    }
                } else if sixth.starts_with("-") {
                    if sixth == "-ic" {
                        ignore_case = true;
                    } else if sixth == "-cs" {
                        ignore_case = false;
                    } else {
                        return Err(ConfigError::InvalidArgument(sixth));
                    }
                } else {
                    // Could be a context count if fifth was a context flag
                    if fifth.starts_with("--") && fifth != "--stats" && fifth != "--s" {
                        context_count = match sixth.parse() {
                            Ok(count) => count,
                            Err(_) => return Err(ConfigError::InvalidContextCount(sixth)),
                        };
                    } else {
                        return Err(ConfigError::InvalidArgument(sixth));
                    }
                }
            } else {
                // Not recursive, handle original cases
                if fourth.starts_with('-') {
                    // it's a flag
                    ignore_case = match fourth.as_str() {
                        "-ic" => true,
                        "-cs" => false,
                        // ignore_case already set from env
                        _ => return Err(ConfigError::InvalidCaseFlag(fourth)),
                    };
                } else {
                    // it's file path 2
                    file_path_2 = fourth;
                }

                if fifth.starts_with('-') {
                    if fifth.starts_with("--") {
                        // It's a context flag
                        context_flag = match fifth.as_str() {
                            "--before" | "--b" => ContextFlag::Before,
                            "--after" | "--a" => ContextFlag::After,
                            "--context" | "--c" => ContextFlag::Context,
                            "--stats" | "--s" => {
                                show_stats = true;
                                ContextFlag::Stats
                            }
                            _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                        };
                    } else {
                        // it's a case sensitivity flag
                        ignore_case = match fifth.as_str() {
                            "-ic" => true,
                            "-cs" => false,
                            // ignore_case already set from env
                            _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
                        };
                    }
                } else {
                    // It could be a context flag name without --
                    context_flag = match fifth.as_str() {
                        "before" | "b" => ContextFlag::Before,
                        "after" | "a" => ContextFlag::After,
                        "context" | "c" => ContextFlag::Context,
                        _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                    };
                }

                if sixth.starts_with("--") {
                    // It's a stats flag
                    show_stats = match sixth.as_str() {
                        "--stats" | "--s" => true,
                        _ => return Err(ConfigError::InvalidContextFlag(sixth)),
                    };
                } else {
                    // Could be a context count
                    context_count = match sixth.parse() {
                        Ok(count) => count,
                        Err(_) => return Err(ConfigError::InvalidContextCount(sixth)),
                    };
                }
            }
        }
        7 => {
            // Format: [binary, query, file_path_1, file_path_2, case_flag, context_flag, context_count]
            // Examples:
            // minigrep "pattern" file1.txt file2.txt -ic --context 2
            // minigrep "pattern" file1.txt file2.txt -cs --before 3

            let fourth = args[3].clone();
            let fifth = args[4].clone();
            let sixth = args[5].clone();
            let seventh = args[6].clone();

            file_path_2 = fourth;

            // Handle case sensitivity flag
            ignore_case = match fifth.as_str() {
                "-ic" => true,
                "-cs" => false,
                // ignore_case already set from env
                _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
            };

            // Handle context flag and stats
            if sixth == "--stats" {
                show_stats = true;
                context_flag = ContextFlag::Stats;
            } else {
                context_flag = match sixth.as_str() {
                    "--before" | "--b" => ContextFlag::Before,
                    "--after" | "--a" => ContextFlag::After,
                    "--context" | "--c" => ContextFlag::Context,
                    "--stats" | "--s" => {
                        show_stats = true;
                        ContextFlag::Stats
                    }
                    _ => return Err(ConfigError::InvalidContextFlag(sixth)),
                };
            }

            // Handle context count (if not stats)
            if !show_stats || context_flag != ContextFlag::Stats {
                context_count = match seventh.parse() {
                    Ok(count) => count,
                    Err(_) => return Err(ConfigError::InvalidContextCount(seventh)),
                };
            }
        }
        8 => {
            // Format: [binary, query, file_path_1, file_path_2, case_flag, context_flag, context_count, --stats]
            // Example:
            // minigrep "pattern" file1.txt file2.txt -ic --context 2 --stats

            let fourth = args[3].clone();
            let fifth = args[4].clone();
            let sixth = args[5].clone();
            let seventh = args[6].clone();
            let eighth = args[7].clone();

            file_path_2 = fourth;

            // Handle case sensitivity flag
            ignore_case = match fifth.as_str() {
                "-ic" => true,
                "-cs" => false,
                // ignore_case already set from env
                _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
            };

            // Handle context flag
            context_flag = match sixth.as_str() {
                "--before" | "--b" => ContextFlag::Before,
                "--after" | "--a" => ContextFlag::After,
                "--context" | "--c" => ContextFlag::Context,
                _ => return Err(ConfigError::InvalidContextFlag(sixth)),
            };

            // Handle context count
            context_count = match seventh.parse() {
                Ok(count) => count,
                Err(_) => return Err(ConfigError::InvalidContextCount(seventh)),
            };

            // Handle stats flag
            if eighth == "--stats" || eighth == "--s" {
                show_stats = true;
            } else {
                return Err(ConfigError::InvalidArgument(eighth));
            }
        }
        _ => return Err(ConfigError::TooManyArguments),
    }
    
    if recursive {
        // Verify file_path_1 is a directory
        let path = std::path::Path::new(&file_path_1);
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
