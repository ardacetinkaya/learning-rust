#![allow(non_snake_case)]
mod cli;
mod task;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};
use task::Task;

fn main() -> std::io::Result<()> {
    let CommandLineArgs { action, file } = CommandLineArgs::from_args();
    let file = file.expect("Failed to find To-Do List ");

    match action {
        Add { task } => task::add(file, Task::new(task)),
        List => task::list(file),
        Remove { position } => task::remove(file, position),
        Complete { position } => task::complete(file, position),
    }
    .expect("Invalid action");

    Ok(())
}

