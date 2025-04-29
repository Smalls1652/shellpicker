use thiserror::Error;

/// Errors for ShellPicker.
#[derive(Error, Debug)]
pub enum ShellPickerError {
    /// Failed to open the config file.
    #[error("Failed to open config file: {0}")]
    ConfigFileError(std::io::Error),

    /// Failed to read/parse the config file.
    #[error("Failed to read config file: {0}")]
    ConfigError(serde_yaml::Error)
}
