use std::collections::BTreeMap;
use std::path::Path;

use crossterm::style::Colorize;

#[derive(Debug, Deserialize)]
pub struct DevloopConfig {
    pub reminders: String,
    pub tasks: Vec<Task>,
    pub actions: BTreeMap<String, Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub command: String,
    #[serde(default)] // bool's default is false
    pub pause: bool,
}

#[derive(Debug, Error)]
pub enum LoadConfigError {
    #[error("load config: {0}")]
    LoadFile(#[from] std::io::Error),

    #[error("parse config: {0}")]
    ParseFile(#[from] toml::de::Error),
}

pub fn load_config(path: &Path) -> Result<DevloopConfig, LoadConfigError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

impl DevloopConfig {
    pub fn help_characters(&self) -> String {
        let keys: Vec<&str> = self
            .actions
            .keys()
            .map(|i| i.as_ref())
            .chain(Some("q"))
            .collect();

        if keys.iter().any(|key| key.len() > 1) {
            keys.join("|")
        } else {
            keys.concat()
        }
    }
}

impl Task {
    pub fn execute(&self) -> bool {
        println!("{}", format!("Running {}...", self.name).on_blue());
        self.run_command()
    }

    #[cfg(windows)]
    fn run_command(&self) -> bool {
        use std::process::Command;
        Command::new("cmd")
            .arg("/C")
            .arg(&self.command)
            .status()
            .expect("child exit status")
            .success()
    }

    #[cfg(unix)]
    fn run_command(&self) -> bool {
        use std::process::Command;
        Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .status()
            .expect("child exit status")
            .success()
    }

    #[cfg(not(any(windows, unix)))]
    fn run_command(&self) -> bool {
        unimplemented!("Unsupported platform");
    }
}
