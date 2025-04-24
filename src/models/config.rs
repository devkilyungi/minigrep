use std::fmt;

#[derive(PartialEq)]
pub enum ContextFlag {
    Before,
    After,
    Context,
    Stats,
}

impl ContextFlag {
    pub fn new(flag: &str) -> Self {
        match flag {
            "before" => Self::Before,
            "after" => Self::After,
            "context" => Self::Context,
            "stats" => Self::Stats,
            _ => panic!("Invalid context flag"),
        }
    }

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

pub struct Config {
    pub query: String,
    pub file_path_1: String,
    pub file_path_2: String, // can be an empty string or contain the second file name
    pub ignore_case: bool,
    pub context_flag: ContextFlag,
    pub context_count: u8,
    pub show_stats: bool,
}
