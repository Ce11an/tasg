use std::fmt;

/// Custom error type for task-related operations
#[derive(Debug)]
pub enum TaskError {
    NotFound(u32),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task with id {} not found", id),
            TaskError::IoError(e) => write!(f, "IO error: {}", e),
            TaskError::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(error: std::io::Error) -> Self {
        TaskError::IoError(error)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(error: serde_json::Error) -> Self {
        TaskError::SerdeError(error)
    }
}
