use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub createdAt: DateTime<Utc>,
    pub isDone: bool,
}

impl Task {
    pub fn new(text: String) -> Task {
        let createdAt: DateTime<Utc> = Utc::now();
        let isDone = false;
        Task {
            text,
            createdAt,
            isDone,
        }
    }
}

impl Display for Task {
    //Implementation of display for a Task
    //Like overriding ToString() method in C#
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let createdAt = self.createdAt.with_timezone(&Local).format("%F %H:%M");
        let status = if self.isDone { "✔︎" } else { " " };
        write!(f, "{:<50} [{}][{}]", self.text, status, createdAt)
    }
}

fn readtasks(mut file: &File) -> std::io::Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn add(filePath: PathBuf, task: Task) -> std::io::Result<()> {
    //Open file as readable and writeable. If file not exists create the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filePath)?;

    //Read content of the file
    let mut tasks = readtasks(&file)?;

    //Add a task into vector
    tasks.push(task);

    //Write into the file as json
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn remove(filePath: PathBuf, position: usize) -> std::io::Result<()> {
    //Open file as readable and writeable
    let file = OpenOptions::new().read(true).write(true).open(filePath)?;

    //Read content of the file
    let mut tasks = readtasks(&file)?;

    //Check if task id is valid
    if position == 0 || position > tasks.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Task ID",
        ));
    }

    //Remove task from vector
    tasks.remove(position - 1);

    file.set_len(0)?;

    //Write into file as json
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list(filePath: PathBuf) -> std::io::Result<()> {
    //Open file as readable
    let file = OpenOptions::new().read(true).open(filePath)?;

    //Read content of the file
    let tasks = readtasks(&file)?;

    //Display content of tasks vector
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        println!("   [Task]{:<40}[Status][Created]", "");
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

pub fn complete(filePath: PathBuf, position: usize) -> std::io::Result<()> {
    //Open file as readable and writeable
    let file = OpenOptions::new().read(true).write(true).open(filePath)?;

    //Read content of the file
    let mut tasks = readtasks(&file)?;

    //Check if task id is valid
    if position == 0 || position > tasks.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Task ID",
        ));
    }
    //Update task value
    tasks[position - 1].isDone = true;

    //Reset content size of file to write updated values
    file.set_len(0)?;

    //Write into file as json
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}
