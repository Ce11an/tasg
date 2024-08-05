//! Command-line interface (CLI)
use clap::{Parser, Subcommand};

/// Command-line interface for the Tasg application.
///
/// The `Cli` struct defines the main entry point for the CLI, using the `clap` crate to parse
/// command-line arguments. It includes the primary command and its associated subcommands.
///
/// The CLI supports various operations such as adding tasks, listing tasks, completing tasks,
/// and deleting tasks.
#[derive(Parser, Debug)]
#[command(name = "tasg", about, version, author)]
pub struct Cli {
    /// The command to execute, specified as a subcommand.
    ///
    /// This field holds the parsed subcommand, which can be one of the variants in the `Commands` enum.
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum representing the available commands in the Tasg CLI.
///
/// The `Commands` enum defines the subcommands supported by the Tasg application. Each variant
/// corresponds to a specific action that the user can perform, such as adding, listing,
/// completing, or deleting tasks.
///
/// # Variants
///
/// - `Add` - Adds a new task with the specified description.
/// - `List` - Lists tasks, with an option to show all tasks, including completed ones.
/// - `Complete` - Marks a task as complete by its ID.
/// - `Delete` - Deletes a task by its ID.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task to the task list.
    ///
    /// This subcommand adds a new task with the provided description.
    ///
    /// # Arguments
    ///
    /// - `description` - A string representing the description of the new task.
    ///
    Add {
        /// The description of the task to add.
        ///
        /// This argument specifies the text description for the new task.
        #[arg()]
        description: String,
    },

    /// List tasks from the task list.
    ///
    /// This subcommand lists tasks with an option to show all tasks, including those that are completed.
    ///
    /// # Arguments
    ///
    /// - `all` - A flag indicating whether to show all tasks. If set, completed tasks will also be listed.
    ///
    List {
        /// Show all tasks, including completed ones.
        #[arg(short, long)]
        all: bool,
    },

    /// Mark a task as complete.
    ///
    /// This subcommand updates the status of the specified task to complete based on its ID.
    ///
    /// # Arguments
    ///
    /// - `id` - The ID of the task to mark as complete. Must be a positive integer.
    Complete {
        /// The ID of the task to complete.
        ///
        /// This argument specifies the ID of the task that should be marked as completed.
        #[arg(value_parser = clap::value_parser!(u32).range(1..))]
        id: u32,
    },

    /// Delete a task from the task list.
    ///
    /// This subcommand removes the task with the specified ID from the task list.
    ///
    /// # Arguments
    ///
    /// - `id` - The ID of the task to delete. Must be a positive integer.
    Delete {
        /// The ID of the task to delete.
        ///
        /// This argument specifies the ID of the task that should be removed from the list.
        #[arg(value_parser = clap::value_parser!(u32).range(1..))]
        id: u32,
    },

    /// Nuke all of the tasks
    ///
    /// This subcommand will delete all your tasks - use with caution!
    Nuke,
}
