#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate thiserror;
extern crate crossterm;
extern crate structopt;

mod args;
mod config;

use crossterm::style::Colorize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{stdin, stdout, Write};

fn main() {
    let args = args::get_opts();
    let config = match config::load_config(&args.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    };
    run_loop(config);
}

fn run_loop(config: config::DevloopConfig) {
    let help_msg = config.help_characters();
    let done_help_msg = format!("Done [{}]:", help_msg).on_green();
    let error_msg = format!("Error. [{}]:", help_msg).on_red();

    'main: loop {
        // Clear on success
        stdout()
            .execute(Clear(ClearType::All))
            .expect("clear terminal")
            .execute(crossterm::cursor::MoveTo(0, 0))
            .expect("reset cursor");

        if execute_tasks(&config.tasks) {
            print!("{}", done_help_msg);
            stdout().flush().expect("flush stdout");
        } else {
            print!("{}", error_msg);
            stdout().flush().expect("flush stdout");
        }

        // Try actions until success
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line).expect("read a line");
            match line.as_str().trim_end() {
                "" => break, // Clear
                "q" => break 'main,
                action_key => {
                    let action = config.actions.get(action_key);
                    if let Some(action) = action {
                        let success = action.execute();
                        if success {
                            if action.pause {
                                stdin().read_line(&mut String::new()).unwrap();
                            }
                            break;
                        // Clear
                        } else {
                            print!("{}", error_msg);
                            stdout().flush().expect("flush stdout");
                            // Choose action again
                        }
                    } else {
                        print!("{} ", "No such action.".on_red());
                        stdout().flush().expect("flush stdout");
                        // Choose action again, use previous prompt for help
                    }
                }
            }
        }
    }
    println!("{}", config.reminders.on_yellow());
}

fn execute_tasks(tasks: &[config::Task]) -> bool {
    for task in tasks {
        if !task.execute() {
            return false;
        }
    }
    true
}
