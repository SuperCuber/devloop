use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::Ordering;

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
    /// Returns Some(status) if task completed, None if interrupted
    pub fn execute(&self) -> Option<bool> {
        println!("{}", format!("Running {}...", self.name).black().on_blue());
        let mut cmd = self.get_command();
        let mut child = cmd.spawn().expect("spawn task");

        super::CTRLC_PRESSED.store(false, Ordering::SeqCst);
        // First check only after 50 millis instead of immediately which usually fails
        std::thread::sleep(std::time::Duration::from_millis(50));
        while !super::CTRLC_PRESSED.load(Ordering::SeqCst) {
            if let Some(status) = child.try_wait().expect("try wait child") {
                return Some(status.success());
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        child.kill().expect("kill task");
        None
    }

    #[cfg(windows)]
    fn get_command(&self) -> Command {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(&self.command);
        cmd
    }

    #[cfg(unix)]
    fn get_command(&self) -> Command {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&self.command);
        cmd
    }

    #[cfg(not(any(windows, unix)))]
    fn get_command(&self) -> bool {
        unimplemented!("Unsupported platform");
    }
}
