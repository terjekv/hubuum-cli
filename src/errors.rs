use hubuum_client::ApiError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Failed to execute command: {0}")]
    CommandExecutionError(String),

    #[error("Error parsing arguments: {0}")]
    ParseError(String),

    #[error("Invalid input")]
    InvalidInput,

    #[error("Invalid option: {0}")]
    InvalidOption(String),

    #[error("Boolean flag options with value: {0:?}")]
    PopulatedFlagOptions(Vec<String>),

    #[error("Integer parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("JSON parse error: {0}")]
    ParseJsonError(#[from] serde_json::Error),

    #[error("Boolean parse error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),

    #[error("Missing required options: {0:?}")]
    MissingOptions(Vec<String>),

    #[error("Duplicate options: {0:?}")]
    DuplicateOptions(Vec<String>),

    #[error("IO error: {0:?}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP Error: {0}")]
    HttpError(String),

    #[error("Regular expression error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("File locking error")]
    LockError,

    #[error("Output format error")]
    FormatError,

    #[error("Error reading configuration file: {0}")]
    ConfigError(String),

    #[error("Failed to initialize configuration: {0}")]
    ConfigurationError(#[from] config::ConfigError),

    #[error("Readline error")]
    ReadlineError(#[from] rustyline::error::ReadlineError),

    #[error("Unable to determine data directory: {0}")]
    DataDirError(String),

    #[error("API error: {0}")]
    ApiError(#[from] ApiError),

    #[error("Multiple entities found: {0}")]
    MultipleEntitiesFound(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Quiet error")]
    Quiet,

    #[error("Jqesque error: {0}")]
    JqesqueError(#[from] jqesque::JqesqueError),

    #[error("Error parsing JSONPath: {0}")]
    JsonPathError(String),
}
