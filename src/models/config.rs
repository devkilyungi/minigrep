use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let ignore_case = if args.len() > 3 {
            let ignore_case = args[3].clone();
            if ignore_case == "-ic" {
                true
            } else if ignore_case == "-cs" {
                false
            } else {
                env::var("IGNORE_CASE").is_ok()
            }
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}