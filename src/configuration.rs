use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use error::DevloopError;
use message::{msg, MessageType};

use yaml_rust::{Yaml, YamlLoader};

pub struct DevloopConfig {
    pub tasks: Vec<Task>,
    pub actions: HashMap<String, Task>,
    pub reminders: String,
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub command: String,
}

pub fn load<S: Into<String>>(filename: S) -> Result<DevloopConfig, DevloopError> {
    let mut contents = String::new();
    File::open(filename.into())?.read_to_string(&mut contents)?;

    let document = &YamlLoader::load_from_str(&contents)?[0];

    println!("document: {:?}", document);

    let tasks = document["tasks"]
        .as_vec()
        .ok_or(DevloopError::InvalidConfig)?
        .into_iter()
        .filter_map(|task| Task::parse(task))
        .collect();

    println!("tasks: {:?}", tasks);

    let actions = document["actions"]
        .as_hash()
        .ok_or(DevloopError::InvalidConfig)?
        .into_iter()
        .filter_map(|(key, value)| {
            Some((key.as_str()?.to_owned(), Task::parse(value)?))
        })
        .collect();

    println!("actions: {:?}", actions);

    let reminders = document["reminders"].as_str().unwrap_or("").to_owned();

    Ok(DevloopConfig {
        tasks: tasks,
        actions: actions,
        reminders: reminders,
    })
}

impl Task {
    fn parse(task: &Yaml) -> Option<Self> {
        let parsed = Task::parse_no_warning(task);
        if parsed.is_none() {
            msg(
                &MessageType::Fail,
                &format!("Warning: failed to parse {:?} as a task", task),
                false,
            );
        }
        parsed
    }

    fn parse_no_warning(task: &Yaml) -> Option<Self> {
        let task = task.as_vec()?;
        Some(Task {
            name: task[0].as_str()?.to_owned(),
            command: task[1].as_str()?.to_owned(),
        })
    }
}
