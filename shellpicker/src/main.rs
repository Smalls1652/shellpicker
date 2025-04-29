use anyhow::{Result, anyhow};
use std::{env, fs, path::PathBuf};

#[cfg(target_family = "unix")]
use std::os::unix::process::CommandExt;

use config::ConfigFile;
use console_utils::{run_shell_picker, write_shell_list};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod config;
mod console_utils;
mod error;

fn main() -> Result<()> {
    let stdout = &mut std::io::stdout();

    let home_dir: PathBuf = match env::consts::OS {
        "windows" => {
            let user_profile = env::var("USERPROFILE")?;

            PathBuf::from(user_profile)
        }

        _ => {
            let home_dir = env::var("HOME")?;

            PathBuf::from(home_dir)
        }
    };

    let config_dir: PathBuf = match env::consts::OS {
        "windows" => home_dir.clone().join("/AppData/Local/shellpicker"),

        _ => home_dir.clone().join(".config/shellpicker"),
    };

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = &config_dir.join("config.yml");

    let config_file = match config_path.exists() {
        false => {
            let base_config = ConfigFile::create_base()?;
            fs::write(config_path, serde_yaml::to_string(&base_config)?)?;

            base_config
        }

        _ => ConfigFile::load_from_path(config_path)?,
    };

    enable_raw_mode()?;

    write_shell_list(stdout, &config_file.shells)?;

    let selected_index = run_shell_picker(stdout, &config_file.shells)?;

    disable_raw_mode()?;

    let selected_shell = config_file.shells[selected_index - 1].clone();

    let mut command = std::process::Command::new(selected_shell.path);
    command.args(selected_shell.args);
    command.current_dir(&home_dir);

    match cfg!(target_os = "windows") {
        true => {
            let proc = command.spawn()?;
            proc.wait_with_output()?;

            Ok(())
        }

        false => {
            let proc_error = command.exec();

            Err(anyhow!("Failed to execute process: {:?}", proc_error))
        }
    }
}
