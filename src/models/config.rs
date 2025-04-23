use std::env;

use super::{ConfigError, ContextFlag};

pub struct Config {
    pub query: String,
    pub file_path_1: String,
    pub file_path_2: String, // can be an empty string or contain the second file name
    pub ignore_case: bool,
    pub context_flag: ContextFlag,
    pub context_count: u8,
    pub show_stats: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ConfigError> {
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

        match args.len() {
            3 => {
                // [binary, query, file_path_1]
                // ignore_case already set from env
            }
            4 => {
                // [binary, query, file_path_1, flag]
                // or [binary, query, file_path_1, context_flag]
                // or [binary, query, file_path_1, file_path_2]

                let fourth = args[3].clone();

                if fourth.starts_with("--") {
                    // it's a context flag
                    context_count = 1;
                    context_flag = match fourth.as_str() {
                        "--before" => ContextFlag::Before,
                        "--after" => ContextFlag::After,
                        "--context" => ContextFlag::Context,
                        "--stats" => {
                            show_stats = true;
                            ContextFlag::Stats
                        },
                        _ => return Err(ConfigError::InvalidContextFlag(fourth)),
                    };
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
                // [binary, query, file_path_1, file_path_2, flag]
                // or [binary, query, file_path_1, file_path_2, context_flag]
                // or [binary, query, file_path_1, context_flag, context_count]

                let fourth = args[3].clone();
                let fifth = args[4].clone();

                if fifth.starts_with("--") {
                    // it's a context flag
                    file_path_2 = fourth;
                    context_count = 1;
                    context_flag = match fifth.as_str() {
                        "--before" => ContextFlag::Before,
                        "--after" => ContextFlag::After,
                        "--context" => ContextFlag::Context,
                        "--stats" => {
                            show_stats = true;
                            ContextFlag::Stats
                        },
                        _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                    };
                } else if fifth.starts_with('-') {
                    // it's a flag
                    file_path_2 = fourth;
                    ignore_case = match fifth.as_str() {
                        "-ic" => true,
                        "-cs" => false,
                        // ignore_case already set from env
                        _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
                    };
                } else {
                    // it's a context count
                    context_flag = match fourth.as_str() {
                        "--before" => ContextFlag::Before,
                        "--after" => ContextFlag::After,
                        "--context" => ContextFlag::Context,
                        "--stats" => {
                            show_stats = true;
                            ContextFlag::Stats
                        },
                        _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                    };
                    context_count = match fifth.parse() {
                        Ok(count) => count,
                        Err(_) => return Err(ConfigError::InvalidContextCount(fifth)),
                    };
                }
            }
            6 => {
                // [binary, query, file_path_1, file_path_2, context_flag, context_count]

                let fourth = args[3].clone();
                let fifth = args[4].clone();
                let sixth = args[5].clone();

                file_path_2 = fourth;
                context_flag = match fifth.as_str() {
                    "--before" => ContextFlag::Before,
                    "--after" => ContextFlag::After,
                    "--context" => ContextFlag::Context,
                    "--stats" => {
                        show_stats = true;
                        ContextFlag::Stats
                    },
                    _ => return Err(ConfigError::InvalidContextFlag(fifth)),
                };
                context_count = match sixth.parse() {
                    Ok(count) => count,
                    Err(_) => return Err(ConfigError::InvalidContextCount(fifth)),
                };
            }
            7 => {
                // [binary, query, file_path_1, file_path_2, flag, context_flag, context_count]

                let fourth = args[3].clone();
                let fifth = args[4].clone();
                let sixth = args[5].clone();
                let seventh = args[6].clone();

                file_path_2 = fourth;
                ignore_case = match fifth.as_str() {
                    "-ic" => true,
                    "-cs" => false,
                    // ignore_case already set from env
                    _ => return Err(ConfigError::InvalidCaseFlag(fifth)),
                };
                context_flag = match sixth.as_str() {
                    "--before" => ContextFlag::Before,
                    "--after" => ContextFlag::After,
                    "--context" => ContextFlag::Context,
                    "--stats" => {
                        show_stats = true;
                        ContextFlag::Stats
                    },
                    _ => return Err(ConfigError::InvalidContextFlag(sixth)),
                };
                context_count = match seventh.parse() {
                    Ok(count) => count,
                    Err(_) => return Err(ConfigError::InvalidContextCount(seventh)),
                };
            }
            _ => return Err(ConfigError::TooManyArguments),
        }

        Ok(Config {
            query,
            file_path_1,
            file_path_2,
            ignore_case,
            context_flag,
            context_count,
            show_stats,
        })
    }
}
