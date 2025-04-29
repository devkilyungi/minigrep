//! Contains configuration models and parsing logic for minigrep.

use std::fmt;

/// Configuration for a minigrep search operation.
///
/// Holds all the parameters that control the search behavior,
/// including query patterns, file paths, and various options.
pub struct Config {
    pub query: String,
    pub file_path_1: String,
    pub file_path_2: String, // can be an empty string or contain the second file name
    pub ignore_case: bool,
    pub context_flag: ContextFlag,
    pub context_count: u8,
    pub show_stats: bool,
    pub recursive: bool,
}

/// Represents the context display mode for search results.
///
/// Determines how many lines before and/or after a match are displayed.
#[derive(PartialEq)]
pub enum ContextFlag {
    /// Display lines before matches
    Before,
    /// Display lines after matches
    After,
    /// Display lines before and after matches
    Context,
    /// Display search statistics
    Stats,
}

impl ContextFlag {
    /// Creates a new ContextFlag from a string representation.
    ///
    /// # Arguments
    ///
    /// * `flag` - A string that should be one of: "before", "after", "context", or "stats"
    ///
    /// # Panics
    ///
    /// Panics if the string doesn't match any of the valid flag values.
    pub fn new(flag: &str) -> Self {
        match flag {
            "before" => Self::Before,
            "after" => Self::After,
            "context" => Self::Context,
            "stats" => Self::Stats,
            _ => panic!("Invalid context flag"),
        }
    }

    /// Returns the string representation of this ContextFlag.
    pub fn as_str(&self) -> &str {
        match self {
            ContextFlag::Before => "before",
            ContextFlag::After => "after",
            ContextFlag::Context => "context",
            ContextFlag::Stats => "stats",
        }
    }
}

impl fmt::Display for ContextFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextFlag::Before => write!(f, "before"),
            ContextFlag::After => write!(f, "after"),
            ContextFlag::Context => write!(f, "context"),
            ContextFlag::Stats => write!(f, "stats"),
        }
    }
}
