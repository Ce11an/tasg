use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use tasg::{
    cli::{Cli, Commands},
    error::TaskError,
    store::{JsonStore, Store},
};

fn get_default_tasks_file() -> PathBuf {
    let mut path = dirs::config_dir().expect("Failed to determine configuration directory");
    path.push("tasg");
    fs::create_dir_all(&path).expect("Failed to create configuration directory");
    path.push("tasks.json");
    path
}

fn run(cli: Cli, store: JsonStore) -> Result<(), TaskError> {
    match cli.command {
        Commands::Add { description } => {
            let id = store.list(true)?.len() as u32 + 1;
            let task = tasg::task::Task::new(id, description);
            store.add(task)?;
            println!("Task added successfully");
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
            println!("Task marked as complete");
        }
        Commands::Delete { id } => {
            store.delete(id)?;
            println!("Task deleted successfully");
        }
    }

    Ok(())
}

fn main() {
    let tasks_file = env::var("TASG_FILE")
        .unwrap_or_else(|_| get_default_tasks_file().to_string_lossy().to_string());
    let store = JsonStore::new(tasks_file);

    let cli = Cli::parse();
    if let Err(e) = run(cli, store) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
