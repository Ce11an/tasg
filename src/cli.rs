use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tasg", about, version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The description of the task
        description: String,
    },
    /// List tasks
    List {
        /// Show all tasks, including completed ones
        #[arg(short, long)]
        all: bool,
    },
    /// Mark a task as complete
    Complete {
        /// The ID of the task to complete
        #[arg(value_parser = clap::value_parser!(u32).range(1..))]
        id: u32,
    },
    /// Delete a task
    Delete {
        /// The ID of the task to delete
        #[arg(value_parser = clap::value_parser!(u32).range(1..))]
        id: u32,
    },
}
