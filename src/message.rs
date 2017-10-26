use std::io::{Write, stdout};

pub enum MessageType {
    Success,
    Fail,
    Info,
    Reminder,
}

pub fn msg(_msg_type: &MessageType, message: &str, is_done: bool) {
    print!("{}{}", message, if is_done { " " } else { "\n" });
    stdout().flush().expect("flush stdout");
}
