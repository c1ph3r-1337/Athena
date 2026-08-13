use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Queued,
    Running,
    Waiting,
    Completed,
    Failed,
    Timeout,
    Cancelled,
    Blocked,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "PENDING"),
            TaskStatus::Queued => write!(f, "QUEUED"),
            TaskStatus::Running => write!(f, "RUNNING"),
            TaskStatus::Waiting => write!(f, "WAITING"),
            TaskStatus::Completed => write!(f, "COMPLETED"),
            TaskStatus::Failed => write!(f, "FAILED"),
            TaskStatus::Timeout => write!(f, "TIMEOUT"),
            TaskStatus::Cancelled => write!(f, "CANCELLED"),
            TaskStatus::Blocked => write!(f, "BLOCKED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,

    // Assignment
    pub assigned_agent: Option<String>,
    pub capabilities_required: HashMap<String, f32>,
    pub token_budget: Option<usize>,

    // DAG dependencies
    pub dependencies: Vec<String>,

    // Execution tracking
    pub process_id: Option<u32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub exit_code: Option<i32>,
    pub token_usage: Option<usize>,

    // Workspace isolation
    pub workspace: Option<String>,
    pub prompt_file: Option<String>,
    pub output_file: Option<String>,
    pub expected_outputs: Vec<String>,

    // Results
    pub result: Option<String>,
    pub error: Option<String>,
    pub logs: Vec<String>,
}

impl Task {
    pub fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            assigned_agent: None,
            capabilities_required: HashMap::new(),
            token_budget: None,
            dependencies: Vec::new(),
            process_id: None,
            start_time: None,
            end_time: None,
            exit_code: None,
            token_usage: None,
            workspace: None,
            prompt_file: None,
            output_file: None,
            expected_outputs: Vec::new(),
            result: None,
            error: None,
            logs: Vec::new(),
        }
    }

    /// Check if all dependencies are satisfied
    pub fn can_run(&self, completed_task_ids: &[String]) -> bool {
        self.status == TaskStatus::Pending
            && self.dependencies.iter().all(|dep| completed_task_ids.contains(dep))
    }

    /// Transition to Running state
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.start_time = Some(chrono::Local::now().to_rfc3339());
    }

    /// Transition to Completed state
    pub fn complete(&mut self, result: String) {
        self.status = TaskStatus::Completed;
        self.end_time = Some(chrono::Local::now().to_rfc3339());
        self.result = Some(result);
    }

    /// Transition to Failed state
    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.end_time = Some(chrono::Local::now().to_rfc3339());
        self.error = Some(error);
    }

    /// Get execution duration as human-readable string
    pub fn duration(&self) -> Option<String> {
        let start = self.start_time.as_ref()?;
        let end = self.end_time.as_ref().map(|s| s.clone()).unwrap_or_else(|| chrono::Local::now().to_rfc3339());
        let start_dt = chrono::DateTime::parse_from_rfc3339(start).ok()?;
        let end_dt = chrono::DateTime::parse_from_rfc3339(&end).ok()?;
        let dur = end_dt.signed_duration_since(start_dt);
        Some(format!("{}s", dur.num_seconds()))
    }
}
