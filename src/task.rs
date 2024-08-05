//! Module containing the task definition.
//!
//! This module defines the core data model for the task management application,
//! encapsulating the `Task` struct, which represents individual tasks in the system.
//! It includes the structure of a task along with methods for creating and managing tasks.

use serde::{Deserialize, Serialize};

/// Represents a task in the system.
///
/// The `Task` struct is the core data model for the task management application. It contains
/// the essential information about a task, including its unique identifier, description, creation
/// and update timestamps, and completion status.
///
/// # Fields
///
/// - `id` - A unique identifier for the task.
/// - `description` - A brief description of the task.
/// - `created_at` - The timestamp when the task was created.
/// - `updated_at` - The timestamp when the task was last updated.
/// - `completed` - A boolean indicating whether the task has been completed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    /// A unique identifier for the task.
    pub id: u32,

    /// A brief description of the task.
    pub description: String,

    /// The timestamp when the task was created.
    pub created_at: chrono::DateTime<chrono::Local>,

    /// The timestamp when the task was last updated.
    pub updated_at: chrono::DateTime<chrono::Local>,

    /// Indicates whether the task has been completed.
    pub completed: bool,
}

impl Task {
    /// Creates a new task with the given ID and description.
    ///
    /// This function initializes a new task with the provided ID and description. The `created_at`
    /// and `updated_at` fields are set to the current local time, and the `completed` field is set
    /// to `false` by default.
    ///
    /// # Arguments
    ///
    /// - `id` - A unique identifier for the task.
    /// - `description` - A brief description of the task.
    ///
    /// # Returns
    ///
    /// A `Task` instance with the provided ID and description, and the current time as the creation and update times.
    pub fn new(id: u32, description: String) -> Self {
        let now = chrono::Local::now();
        Self { id, description, created_at: now, updated_at: now, completed: false }
    }
}
