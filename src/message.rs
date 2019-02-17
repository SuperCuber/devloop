use colored::{ColoredString, Colorize};

use std::io::{stdout, Write};

pub enum MessageType {
    Success,
    Fail,
    Info,
    Reminder,
}

impl MessageType {
    fn color(&self, message: &str) -> ColoredString {
        match *self {
            MessageType::Success => message.on_green(),
            MessageType::Fail => message.on_red(),
            MessageType::Info => message.on_blue(),
            MessageType::Reminder => message.on_yellow(),
        }.black()
    }
}

pub fn msg(msg_type: &MessageType, message: &str, is_done: bool) {
    print!(
        "{}{}",
        msg_type.color(message),
        if is_done { " " } else { "\n" }
    );
    stdout().flush().expect("flush stdout");
}
