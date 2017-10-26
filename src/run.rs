use std::process::Command;

use configuration::{DevloopConfig, Task};
use message::{msg, MessageType};

impl DevloopConfig {
    pub fn run(&self) {
        let help = self.calculate_help();
        loop {
            clear_screen();
            if self.run_tasks() {
                msg(&MessageType::Success, &format!("Done [{}]:", help), true);
            } else {
                msg(&MessageType::Fail, "Error.", false);
            }
            match read_line().as_ref() {
                "" => continue,
                "q" => break,
                action => {
                    if let Some(task) = self.actions.get(action) {
                        if !task.execute() {
                            msg(&MessageType::Fail, "Error.", false);
                            read_line();
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
        msg(&MessageType::Reminder, &self.reminders, false);
    }

    fn calculate_help(&self) -> String {
        let keys: Vec<&String> = self.actions.keys().collect();

        if keys.iter().any(|key| key.len() > 1) {
            keys.iter().fold(String::new(), |a, i| a + "|" + i)
        } else {
            keys.iter().fold(String::new(), |a, i| a + i)
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

fn clear_screen() {
    Command::new("sh").args(&["-c", "clear"]).status().expect(
        "clear screen",
    );
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

        Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .status()
            .expect("child exit status")
            .success()
    }
}
