use std::{error, fmt};

#[derive(Debug)]
pub enum ConfigError {
    NotEnoughArguments,
    TooManyArguments,
    InvalidFlag(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotEnoughArguments => write!(f, "Not enough arguments"),
            ConfigError::TooManyArguments => write!(f, "Too many arguments"),
            ConfigError::InvalidFlag(flag) => write!(f, "Invalid flag: '{}'", flag),
        }
    }
}

impl error::Error for ConfigError {}
