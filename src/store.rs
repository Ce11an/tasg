//! JSON-based Task Storage
//!
//! This module provides a JSON-based implementation of the `Store` trait for managing tasks in a task management CLI application.
//! Tasks are stored in a JSON file, and operations such as adding, listing, completing, and deleting tasks are supported.

use crate::error::TaskError;
use crate::task::Task;

/// Trait defining the operations for task storage.
///
/// The `Store` trait abstracts the operations that can be performed on task data, such as adding, listing, completing, and deleting tasks.
pub trait Store {
    /// Adds a new task to the store.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to be added.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully added, or a `TaskError` if an error occurs.
    fn add(&self, task: Task) -> Result<(), TaskError>;

    /// Lists all tasks or only incomplete tasks.
    ///
    /// # Arguments
    ///
    /// * `all` - If true, lists all tasks. If false, lists only incomplete tasks.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Task>, TaskError>` - Returns a vector of tasks, or a `TaskError` if an error occurs.
    fn list(&self, all: bool) -> Result<Vec<Task>, TaskError>;

    /// Marks a task as complete.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to be marked as complete.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully marked as complete, or a `TaskError` if an error occurs.
    fn complete(&self, id: u32) -> Result<(), TaskError>;

    /// Deletes a task from the store.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to be deleted.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully deleted, or a `TaskError` if an error occurs.
    fn delete(&self, id: u32) -> Result<(), TaskError>;

    /// Path to the store.
    ///
    /// # Returns
    ///
    /// * `&str` containing the file path to the store.
    fn path(&self) -> &str;

    /// Edits an existing task's description.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to edit.
    /// * `description` - The new description of the task. If `None`, the description remains unchanged.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully edited, or a `TaskError` if the task is not found.
    fn edit(&self, id: u32, description: Option<String>) -> Result<(), TaskError>;
}

/// JSON-based implementation of the `Store` trait.
///
/// The `JsonStore` struct provides a JSON-based storage mechanism for tasks. Tasks are stored in a JSON file,
/// and operations such as adding, listing, completing, and deleting tasks are supported.
#[derive(Debug)]
pub struct JsonStore {
    /// The path to the JSON file where tasks are stored.
    path: String,
}

impl JsonStore {
    /// Creates a new `JsonStore` with the given file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string or any type that can be converted into a string representing the path to the JSON file.
    ///
    /// # Returns
    ///
    /// * `JsonStore` - A new instance of `JsonStore`.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    /// Loads tasks from the JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Task>, TaskError>` - Returns a vector of tasks loaded from the JSON file, or a `TaskError` if an error occurs.
    fn load(&self) -> Result<Vec<Task>, TaskError> {
        let path = std::path::Path::new(&self.path);
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Vec::new())
        }
    }

    /// Saves tasks to the JSON file.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A slice of tasks to be saved to the JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the tasks are successfully saved, or a `TaskError` if an error occurs.
    fn save(&self, tasks: &[Task]) -> Result<(), TaskError> {
        let data = serde_json::to_string(tasks)?;
        Ok(std::fs::write(&self.path, data)?)
    }
}

impl Store for JsonStore {
    /// Adds a new task to the JSON store.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to be added.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully added, or a `TaskError` if an error occurs.
    fn add(&self, task: Task) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        tasks.push(task);
        self.save(&tasks)
    }

    /// Lists all tasks or only incomplete tasks.
    ///
    /// # Arguments
    ///
    /// * `all` - If true, lists all tasks. If false, lists only incomplete tasks.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Task>, TaskError>` - Returns a vector of tasks, or a `TaskError` if an error occurs.
    fn list(&self, all: bool) -> Result<Vec<Task>, TaskError> {
        let tasks = self.load()?;
        Ok(if all { tasks } else { tasks.into_iter().filter(|t| !t.completed).collect() })
    }

    /// Marks a task as complete in the JSON store.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to be marked as complete.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully marked as complete, or a `TaskError` if the task is not found.
    fn complete(&self, id: u32) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            self.save(&tasks)
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    /// Deletes a task from the JSON store.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to be deleted.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskError>` - Returns `Ok(())` if the task is successfully deleted, or a `TaskError` if the task is not found.
    fn delete(&self, id: u32) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        let initial_len = tasks.len();
        tasks.retain(|task| task.id != id);
        if tasks.len() < initial_len {
            self.save(&tasks)
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    /// Path to the store.
    ///
    /// # Returns
    ///
    /// * `&str` containing the file path to the store.
    fn path(&self) -> &str {
        &self.path
    }

    fn edit(&self, id: u32, description: Option<String>) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            if let Some(new_description) = description {
                task.description = new_description;
            }
            task.updated_at = chrono::Local::now();
            self.save(&tasks)
        } else {
            Err(TaskError::NotFound(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::TaskError;
    use crate::task::Task;
    use std::fs;
    use tempfile::tempdir;

    /// Tests the `add` method of `JsonStore`.
    ///
    /// This test verifies that a task can be successfully added to the JSON store.
    #[test]
    fn test_add_task() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task = Task::new(1, String::from("Test task"));
        store.add(task).unwrap();

        let data = fs::read_to_string(&store.path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[0].description, "Test task");
        assert!(!tasks[0].completed);
    }

    /// Tests the `list` method of `JsonStore`.
    ///
    /// This test verifies that tasks can be successfully listed from the JSON store.
    #[test]
    fn test_list_tasks() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task1 = Task::new(1, String::from("Test task 1"));
        let task2 = Task::new(2, String::from("Test task 2"));
        store.add(task1).unwrap();
        store.add(task2).unwrap();

        let all_tasks = store.list(true).unwrap();
        assert_eq!(all_tasks.len(), 2);

        let incomplete_tasks = store.list(false).unwrap();
        assert_eq!(incomplete_tasks.len(), 2);
        assert_eq!(incomplete_tasks[0].id, 1);
    }

    /// Tests the `complete` method of `JsonStore`.
    ///
    /// This test verifies that a task can be successfully marked as complete in the JSON store.
    #[test]
    fn test_complete_task() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task = Task::new(1, String::from("Test task"));
        store.add(task).unwrap();
        store.complete(1).unwrap();

        let data = fs::read_to_string(&store.path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].completed);
    }

    /// Tests the `complete` method of `JsonStore` when the task is not found.
    ///
    /// This test verifies that an error is returned when attempting to mark a non-existent task as complete.
    #[test]
    fn test_complete_task_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let result = store.complete(1);
        assert!(result.is_err());
        if let Err(TaskError::NotFound(id)) = result {
            assert_eq!(id, 1);
        } else {
            panic!("Expected TaskError::NotFound");
        }
    }

    /// Tests the `delete` method of `JsonStore`.
    ///
    /// This test verifies that a task can be successfully deleted from the JSON store.
    #[test]
    fn test_delete_task() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task = Task::new(1, String::from("Test task"));
        store.add(task).unwrap();
        store.delete(1).unwrap();

        let data = fs::read_to_string(&store.path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        assert_eq!(tasks.len(), 0);
    }

    /// Tests the `delete` method of `JsonStore` when the task is not found.
    ///
    /// This test verifies that an error is returned when attempting to delete a non-existent task.
    #[test]
    fn test_delete_task_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let result = store.delete(1);
        assert!(result.is_err());
        if let Err(TaskError::NotFound(id)) = result {
            assert_eq!(id, 1);
        } else {
            panic!("Expected TaskError::NotFound");
        }
    }

    /// Tests the `edit` method of `JsonStore`.
    ///
    /// This test verifies that a task's description can be successfully edited in the JSON store.
    #[test]
    fn test_edit_task() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task = Task::new(1, String::from("Original task"));
        store.add(task).unwrap();

        store.edit(1, Some("Edited task".to_string())).unwrap();

        let data = fs::read_to_string(&store.path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Edited task");
    }

    /// Tests the `edit` method of `JsonStore` when the task is not found.
    ///
    /// This test verifies that an error is returned when attempting to edit a non-existent task.
    #[test]
    fn test_edit_task_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let result = store.edit(1, Some("New description".to_string()));
        assert!(result.is_err());
        if let Err(TaskError::NotFound(id)) = result {
            assert_eq!(id, 1);
        } else {
            panic!("Expected TaskError::NotFound");
        }
    }

    /// Tests the `edit` method of `JsonStore` when no description is provided.
    ///
    /// This test verifies that a task's description does not change if an description is not
    /// provided.
    #[test]
    fn test_edit_task_no_description() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let store = JsonStore::new(file_path.to_str().unwrap().to_string());

        let task = Task::new(1, String::from("Original task"));
        store.add(task).unwrap();

        store.edit(1, None).unwrap();

        let data = fs::read_to_string(&store.path).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Original task");
    }
}
