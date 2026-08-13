use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::task::{Task, TaskStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationState {
    pub session_id: String,
    pub project_name: String,
    pub objective: String,
    pub tasks: Vec<Task>,
    pub agents_used: Vec<String>,
    pub started_at: String,
    pub updated_at: String,
    pub phase: OrchestrationPhase,
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrchestrationPhase {
    Planning,
    Validating,
    Executing,
    WaitingForCompletion,
    Integration,
    FinalValidation,
    Complete,
    Failed,
}

impl OrchestrationState {
    pub fn new(session_id: String, project_name: String, objective: String) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            session_id,
            project_name,
            objective,
            tasks: Vec::new(),
            agents_used: Vec::new(),
            started_at: now.clone(),
            updated_at: now,
            phase: OrchestrationPhase::Planning,
            output_dir: None,
        }
    }

    pub fn save(&self, dir: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(dir.join("state.json"), content)?;
        Ok(())
    }

    pub fn load(dir: &Path) -> Result<Self> {
        let content = fs::read_to_string(dir.join("state.json"))?;
        let state: Self = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn update_task(&mut self, task_id: &str, status: TaskStatus) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status;
            self.updated_at = chrono::Local::now().to_rfc3339();
        }
    }
}
