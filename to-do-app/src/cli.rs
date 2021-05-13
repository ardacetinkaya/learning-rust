use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        task: String
    },
    List,
    Remove { 
        #[structopt()]
        position: usize 
    },
    Complete { 
        #[structopt()]
        position: usize 
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name="A simple To-Do List with Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub file: Option<PathBuf>
}
