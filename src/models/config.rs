use std::env;

use super::ConfigError;

pub struct Config {
    pub query: String,
    pub file_path_1: String,
    pub file_path_2: String,
    pub ignore_case: bool,
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

        match args.len() {
            3 => {
                // [binary, query, file_path_1]
                // ignore_case already set from env
            }
            4 => {
                let fourth = args[3].clone();
                if fourth.starts_with('-') {
                    // it's a flag
                    ignore_case = match fourth.as_str() {
                        "-ic" => true,
                        "-cs" => false,
                        _ => return Err(ConfigError::InvalidFlag(fourth)),
                    };
                } else {
                    // it's a second file
                    file_path_2 = fourth;
                }
            }
            5 => {
                file_path_2 = args[3].clone();
                let fifth = args[4].clone();
                ignore_case = match fifth.as_str() {
                    "-ic" => true,
                    "-cs" => false,
                    _ => env::var("IGNORE_CASE").is_ok(),
                };
            }
            _ => return Err(ConfigError::TooManyArguments),
        }

        Ok(Config {
            query,
            file_path_1,
            file_path_2,
            ignore_case,
        })
    }
}
