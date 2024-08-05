//! Error Handling for Task Operations
//!
//! This module defines the custom error type `TaskError` used for handling errors
//! in task-related operations within the task management CLI application.

use std::fmt;

/// Custom error type for task-related operations.
///
/// The `TaskError` enum encapsulates various errors that can occur while managing tasks,
/// including task not found errors, I/O errors, serialization/deserialization errors, and invalid input.
#[derive(Debug)]
pub enum TaskError {
    /// Error indicating that a task with the specified ID was not found.
    ///
    /// # Fields
    ///
    /// * `u32` - The ID of the task that was not found.
    NotFound(u32),

    /// Error representing an I/O operation failure.
    ///
    /// This variant wraps a standard `std::io::Error`.
    ///
    /// # Fields
    ///
    /// * `std::io::Error` - The I/O error that occurred.
    IoError(std::io::Error),

    /// Error representing a serialization/deserialization failure.
    ///
    /// This variant wraps a `serde_json::Error` from the `serde_json` crate.
    ///
    /// # Fields
    ///
    /// * `serde_json::Error` - The serialization or deserialization error that occurred.
    SerdeError(serde_json::Error),

    /// Error representing invalid input.
    ///
    /// # Fields
    ///
    /// * `String` - Message stating why input is invalid.
    InvalidInput(String),
}

impl fmt::Display for TaskError {
    /// Formats the `TaskError` for display purposes.
    ///
    /// This implementation provides a user-friendly description of the error, which is useful
    /// for displaying error messages to the end-user.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to write the formatted string.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task with ID {} not found", id),
            TaskError::IoError(e) => write!(f, "I/O error - {}", e),
            TaskError::SerdeError(e) => write!(f, "Serialization error -  {}", e),
            TaskError::InvalidInput(msg) => write!(f, "Invalid input - {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<std::io::Error> for TaskError {
    /// Converts a `std::io::Error` into a `TaskError`.
    ///
    /// This implementation allows I/O errors to be seamlessly converted into `TaskError::IoError` variants,
    /// enabling better error handling in task-related operations.
    ///
    /// # Arguments
    ///
    /// * `error` - The `std::io::Error` to convert.
    ///
    /// # Returns
    ///
    /// * `TaskError` - The corresponding `TaskError` variant.
    fn from(error: std::io::Error) -> Self {
        TaskError::IoError(error)
    }
}

impl From<serde_json::Error> for TaskError {
    /// Converts a `serde_json::Error` into a `TaskError`.
    ///
    /// This implementation allows serialization and deserialization errors to be seamlessly converted
    /// into `TaskError::SerdeError` variants, enabling better error handling in task-related operations.
    ///
    /// # Arguments
    ///
    /// * `error` - The `serde_json::Error` to convert.
    ///
    /// # Returns
    ///
    /// * `TaskError` - The corresponding `TaskError` variant.
    fn from(error: serde_json::Error) -> Self {
        TaskError::SerdeError(error)
    }
}
