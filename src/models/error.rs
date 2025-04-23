use std::{error, fmt};

#[derive(Debug)]
pub enum ConfigError {
    NotEnoughArguments,
    TooManyArguments,
    InvalidCaseFlag(String),
    InvalidContextFlag(String),
    InvalidContextCount(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotEnoughArguments => write!(f, "Not enough arguments"),
            ConfigError::TooManyArguments => write!(f, "Too many arguments"),
            ConfigError::InvalidCaseFlag(flag) => write!(f, "Invalid case flag: '{}'", flag),
            ConfigError::InvalidContextFlag(flag) => write!(f, "Invalid context flag: '{}'", flag),
            ConfigError::InvalidContextCount(count) => write!(f, "Invalid context count: '{}'", count),
        }
    }
}

impl error::Error for ConfigError {}
