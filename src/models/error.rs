//! Contains a custom error model with variants returned after argument parsing results in an error

use std::{error, fmt};

/// Errors that can occur during command-line argument parsing.
///
/// These errors represent the various ways that user-provided command-line
/// arguments might be invalid when configuring the minigrep tool.
#[derive(Debug)]
pub enum ConfigError {
    NotEnoughArguments,
    TooManyArguments,
    InvalidCaseFlag(String),
    InvalidContextFlag(String),
    InvalidContextCount(String),
    InvalidArgument(String),
    NotADirectory(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotEnoughArguments => write!(f, "Not enough arguments"),
            ConfigError::TooManyArguments => write!(f, "Too many arguments"),
            ConfigError::InvalidCaseFlag(flag) => write!(f, "Invalid case flag: '{}'", flag),
            ConfigError::InvalidContextFlag(flag) => write!(f, "Invalid context flag: '{}'", flag),
            ConfigError::InvalidContextCount(count) => {
                write!(f, "Invalid context count: '{}'", count)
            }
            ConfigError::InvalidArgument(arg) => write!(f, "Invalid argument: '{}'", arg),
            ConfigError::NotADirectory(path) => {
                write!(f, "Path provided is not a directory: '{}'", path)
            }
        }
    }
}

impl error::Error for ConfigError {}
