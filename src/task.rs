use serde::{Deserialize, Serialize};

/// Represents a task in the system
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
    pub completed: bool,
}

impl Task {
    /// Creates a new task with the given id and description
    pub fn new(id: u32, description: String) -> Self {
        let now = chrono::Local::now();
        Self { id, description, created_at: now, updated_at: now, completed: false }
    }
}
