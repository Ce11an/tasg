use crate::error::TaskError;
use crate::task::Task;
use std::path::Path;

/// Trait defining the operations for task storage
pub trait Store {
    fn add(&self, task: Task) -> Result<(), TaskError>;
    fn list(&self, all: bool) -> Result<Vec<Task>, TaskError>;
    fn delete(&self, id: u32) -> Result<(), TaskError>;
    fn complete(&self, id: u32) -> Result<(), TaskError>;
}

/// JSON-based implementation of the Store trait
#[derive(Debug)]
pub struct JsonStore {
    path: String,
}

impl JsonStore {
    /// Creates a new JsonStore with the given file path
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    fn load(&self) -> Result<Vec<Task>, TaskError> {
        let path = Path::new(&self.path);
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Vec::new())
        }
    }

    fn save(&self, tasks: &[Task]) -> Result<(), TaskError> {
        let data = serde_json::to_string(tasks)?;
        Ok(std::fs::write(&self.path, data)?)
    }
}

impl Store for JsonStore {
    fn add(&self, task: Task) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        tasks.push(task);
        self.save(&tasks)
    }

    fn list(&self, all: bool) -> Result<Vec<Task>, TaskError> {
        let tasks = self.load()?;
        Ok(if all { tasks } else { tasks.into_iter().filter(|t| !t.completed).collect() })
    }

    fn complete(&self, id: u32) -> Result<(), TaskError> {
        let mut tasks = self.load()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            self.save(&tasks)
        } else {
            Err(TaskError::NotFound(id))
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::TaskError;
    use crate::task::Task;
    use std::fs;
    use tempfile::tempdir;

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
}
