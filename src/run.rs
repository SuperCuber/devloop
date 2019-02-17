use std::process::Command;
use crossterm_terminal::{terminal, ClearType};

use configuration::{DevloopConfig, Task};
use message::{msg, MessageType};

impl DevloopConfig {
    pub fn run(&self) {
        let help = self.calculate_help();
        'main: loop {
            terminal().clear(ClearType::All).expect("clear terminal");
            if self.run_tasks() {
                msg(&MessageType::Success, &format!("Done [{}]:", help), true);
            } else {
                msg(&MessageType::Fail, "Error.", false);
            }
            loop {
                match read_line().as_ref() {
                    "q" => break 'main,
                    action => {
                        if self.actions
                            .get(action)
                            .map(|task| task.execute())
                            .unwrap_or(true)
                        {
                            break;
                        } else {
                            msg(&MessageType::Fail, "Error.", false);
                            // Read line again
                        }
                    }
                }
            }
        }
        msg(&MessageType::Reminder, &self.reminders, false);
    }

    fn calculate_help(&self) -> String {
        let keys: Vec<&str> = self.actions
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

    fn run_tasks(&self) -> bool {
        for task in &self.tasks {
            if !task.execute() {
                return false;
            }
        }
        true
    }
}

fn read_line() -> String {
    use std::io::BufRead;
    let stdin = ::std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line.clone()
}

impl Task {
    fn execute(&self) -> bool {
        msg(
            &MessageType::Info,
            &format!("Running {}...", self.name),
            false,
        );

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(&self.command)
                .status()
                .expect("child exit status")
                .success()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .status()
                .expect("child exit status")
                .success()
        }
    }
}
