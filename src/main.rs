//! Manage your tasks with `tasg`!
//!
//! `tasg` is a command-line tool for managing tasks. It provides functionalities to add, list, complete, and delete tasks. The tasks are stored in a JSON file located in the user's configuration directory.

use std::io::{self, Write};

use clap::Parser;
use tasg::{
    cli::{Cli, Commands},
    error::TaskError,
    store::{JsonStore, Store},
};

/// Gets the default path for the tasks file.
///
/// This function determines the path to the tasks JSON file, which is located in the user's configuration directory (e.g., `~/.config/tasg/tasks.json` on Linux).
///
/// # Returns
///
/// * A `PathBuf` containing the path to the tasks JSON file.
///
/// # Panics
///
/// * If the configuration directory cannot be determined.
/// * If the tasks directory or file cannot be created.
fn get_default_tasks_file() -> std::path::PathBuf {
    let mut path = dirs::config_dir().expect("Failed to determine configuration directory");
    path.push("tasg");
    std::fs::create_dir_all(&path).expect("Failed to create configuration directory");
    path.push("tasks.json");
    path
}

/// Ensures that the tasks file exists.
///
/// This function checks if the tasks JSON file exists at the given path. If it does not exist, the function creates the necessary directories and an empty tasks file.
///
/// # Arguments
///
/// * `path` - A string slice representing the path to the tasks file.
///
/// # Returns
///
/// * `Result<(), TaskError>` - Returns `Ok(())` if the file exists or is successfully created. Returns a `TaskError` if there is a problem creating the file or directory.
///
/// # Errors
///
/// * This function will return an error if the directories or file cannot be created.
fn ensure_tasks_file_exists(path: &str) -> Result<(), TaskError> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::File::create(path)?;
        std::fs::write(path, "[]")?;
    }
    Ok(())
}

/// Runs the CLI commands provided by the user.
///
/// This function executes the command specified by the user via the CLI. The available commands are `Add`, `List`, `Complete`, and `Delete`.
///
/// # Arguments
///
/// * `cli` - A `Cli` struct containing the parsed command-line arguments.
/// * `store` - A `JsonStore` instance responsible for managing the tasks data.
///
/// # Returns
///
/// * `Result<(), TaskError>` - Returns `Ok(())` if the command executes successfully. Returns a `TaskError` if an error occurs during command execution.
///
/// # Errors
///
/// * This function will return an error if there is an issue with adding, listing, completing, or deleting a task.
fn run(cli: Cli, store: JsonStore) -> Result<(), TaskError> {
    match cli.command {
        Commands::Add { description } => {
            if description.trim().is_empty() {
                return Err(TaskError::InvalidInput("Description cannot be empty".into()));
            }
            let id = store.list(true)?.len() as u32 + 1;
            let task = tasg::task::Task::new(id, description);
            store.add(task)?;
        }
        Commands::List { all } => {
            let tasks = store.list(all)?;
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                println!(
                    "{:<5} {:<50} {:<20} {}",
                    "ID",
                    "Description",
                    "Created At",
                    if all { "Completed" } else { "" }
                );
                for task in tasks {
                    println!(
                        "{:<5} {:<50} {:<20} {}",
                        task.id,
                        task.description,
                        task.created_at.format("%Y-%m-%d %H:%M:%S"),
                        if all {
                            if task.completed {
                                "Yes"
                            } else {
                                "No"
                            }
                        } else {
                            ""
                        }
                    );
                }
            }
        }
        Commands::Complete { id } => {
            store.complete(id)?;
        }
        Commands::Delete { id } => {
            store.delete(id)?;
        }
        Commands::Nuke => {
            print!(
                "Are you sure you want to delete all tasks? This action cannot be undone. (y/N): "
            );
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                std::fs::remove_file(store.path())?;
                println!("All tasks have been deleted.");
            } else {
                println!("Operation cancelled.");
            }
        }
    }

    Ok(())
}

/// The main entry point for the `tasg` application.
///
/// This function is responsible for initializing the application, parsing command-line arguments, and invoking the appropriate command handler.
///
/// # Process
///
/// 1. Determines the tasks file path. If the `TASG_FILE` environment variable is set, its value is used. Otherwise, the default path (`~/.config/tasg/tasks.json`) is used.
/// 2. Ensures that the tasks file exists by calling `ensure_tasks_file_exists`.
/// 3. Creates a `JsonStore` to manage task data in the JSON file.
/// 4. Parses the command-line arguments using `Cli::parse`.
/// 5. Calls `run` to execute the command provided by the user.
/// 6. Handles any errors that occur during execution and prints appropriate error messages.
///
/// # Panics
///
/// * If the tasks file path cannot be determined or created.
/// * If the application encounters an error while running.
fn main() {
    let tasks_file = std::env::var("TASG_FILE")
        .unwrap_or_else(|_| get_default_tasks_file().to_string_lossy().to_string());

    if let Err(e) = ensure_tasks_file_exists(&tasks_file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let store = JsonStore::new(tasks_file);

    let cli = Cli::parse();
    if let Err(e) = run(cli, store) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
