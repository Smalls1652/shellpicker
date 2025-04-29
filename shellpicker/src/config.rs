use std::{env, ffi::OsStr, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::ShellPickerError;

/// Config file for ShellPicker.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigFile {
    /// Shells to choose from.
    pub shells: Vec<ShellItem>
}

impl ConfigFile {
    /// Load the config file from the given path.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the config file.
    pub fn load_from_path(path: &PathBuf) -> Result<Self, ShellPickerError> {
        path.try_exists()
            .map_err(|e| ShellPickerError::ConfigFileError(e))?;

        let file = File::open(&path)
            .map_err(|e| ShellPickerError::ConfigFileError(e))?;
        let config: ConfigFile = serde_yaml::from_reader(file)
            .map_err(|e| ShellPickerError::ConfigError(e))?;

        Ok(config)
    }

    /// Create the base config file.
    pub fn create_base() -> Result<Self, env::VarError> {
        match env::consts::OS {
            "windows" => {
                Ok(
                    ConfigFile {
                        shells: vec![
                            ShellItem::new("PowerShell", PathBuf::from("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"), vec![]),
                            ShellItem::new("Command Prompt", PathBuf::from("C:\\Windows\\System32\\cmd.exe"), vec![]),
                        ]
                    }
                )
            }

            _ => {
                let default_shell = env::var("SHELL")?;

                let default_shell_path = PathBuf::from(default_shell);
                let default_shell_filename = &default_shell_path
                    .file_name()
                    .unwrap_or_else(|| OsStr::new("Default"))
                    .to_string_lossy()
                    .to_string();

                Ok(
                    ConfigFile {
                        shells: vec![
                            ShellItem::new(default_shell_filename, default_shell_path, vec![])
                        ]
                    }
                )
            }
        }
    }
}

/// Represents a configured shell to choose from.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShellItem {
    /// The name of the shell.
    pub name: String,

    /// The path to the shell.
    pub path: PathBuf,

    /// Arguments to supply to the shell.
    pub args: Vec<String>
}

impl ShellItem {
    /// Initialize a new `ShellItem`.
    /// 
    /// ## Arguments
    /// 
    /// * `name` - The name of the shell.
    /// * `path` - The path to the shell.
    /// * `args` - Arguments to supply to the shell.
    pub fn new(name: &str, path: PathBuf, args: Vec<&str>) -> Self {
        ShellItem {
            name: name.to_string(),
            path,
            args: args.iter().map(|&arg| arg.to_string()).collect(),
        }
    }
}
