extern crate crossterm;
extern crate ctrlc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate thiserror;

mod args;
mod config;

use crossterm::style::Colorize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;

use std::io::{stdin, stdout, Write};
use std::sync::atomic::AtomicBool;

lazy_static! {
    pub(crate) static ref CTRLC_PRESSED: AtomicBool = AtomicBool::new(false);
}

fn main() {
    let args = args::get_opts();

    ctrlc::set_handler(ctrl_c_handler).expect("set ctrl+c handler");

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
    let done_help_msg = format!("Done [{}]:", help_msg).black().on_green();
    let error_msg = format!("Error. [{}]:", help_msg).black().on_red();
    let interrupted_msg = format!("Interrupted. [{}]:", help_msg).black().on_red();

    'main: loop {
        // Clear on success
        stdout()
            .execute(Clear(ClearType::All))
            .expect("clear terminal")
            .execute(crossterm::cursor::MoveTo(0, 0))
            .expect("reset cursor");

        match execute_tasks(&config.tasks) {
            Some(true) => {
                print!("{}", done_help_msg);
                stdout().flush().expect("flush stdout");
            }
            Some(false) => {
                print!("{}", error_msg);
                stdout().flush().expect("flush stdout");
            }
            None => {
                print!("{}", interrupted_msg);
                stdout().flush().expect("flush stdout");
            }
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
                        match action.execute() {
                            Some(true) => {
                                if action.pause {
                                    stdin().read_line(&mut String::new()).unwrap();
                                }
                                break;
                                // Clear
                            }
                            Some(false) => {
                                print!("{}", error_msg);
                                stdout().flush().expect("flush stdout");
                                // Choose action again
                            }
                            None => {
                                print!("\n{}", interrupted_msg);
                                stdout().flush().expect("flush stdout");
                            }
                        }
                    } else {
                        print!("{} ", "No such action.".black().on_red());
                        stdout().flush().expect("flush stdout");
                        // Choose action again, use previous prompt for help
                    }
                }
            }
        }
    }
    println!("{}", config.reminders.black().on_yellow());
}

fn execute_tasks(tasks: &[config::Task]) -> Option<bool> {
    for task in tasks {
        if !task.execute()? {
            return Some(false);
        }
    }
    Some(true)
}

fn ctrl_c_handler() {
    use std::sync::atomic::Ordering;
    CTRLC_PRESSED.store(true, Ordering::SeqCst);
}
