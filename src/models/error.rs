#[derive(Debug)]
pub enum ConfigError {
    NotEnoughArguments,
    TooManyArguments,
    InvalidFlag(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotEnoughArguments => write!(f, "Not enough arguments"),
            ConfigError::TooManyArguments => write!(f, "Too many arguments"),
            ConfigError::InvalidFlag(flag) => write!(f, "Invalid flag: {}", flag),
        }
    }
}

impl std::error::Error for ConfigError {}